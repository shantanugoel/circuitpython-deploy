use crate::error::{CpdError, Result};
use ignore::{Match, gitignore::GitignoreBuilder};
use std::path::{Path, PathBuf};

pub struct IgnoreFilter {
    gitignore: ignore::gitignore::Gitignore,
    force_include: Option<ignore::gitignore::Gitignore>,
    project_root: PathBuf,
}

impl IgnoreFilter {
    pub fn new(project_root: &Path) -> Result<Self> {
        let mut builder = GitignoreBuilder::new(project_root);

        // Add default ignores
        Self::add_default_patterns(&mut builder)?;

        // Add .cpdignore if it exists
        let cpdignore_path = project_root.join(".cpdignore");
        if cpdignore_path.exists() {
            builder.add(&cpdignore_path);
        }

        // Add .gitignore if it exists (as additional patterns)
        let gitignore_path = project_root.join(".gitignore");
        if gitignore_path.exists() {
            builder.add(&gitignore_path);
        }

        let gitignore = builder
            .build()
            .map_err(|e| CpdError::InvalidIgnorePattern {
                pattern: e.to_string(),
            })?;

        // Build force include patterns from .cpdforce if it exists
        let force_include = {
            let cpdforce_path = project_root.join(".cpdforce");
            if cpdforce_path.exists() {
                let mut force_builder = GitignoreBuilder::new(project_root);
                force_builder.add(&cpdforce_path);
                Some(
                    force_builder
                        .build()
                        .map_err(|e| CpdError::InvalidIgnorePattern {
                            pattern: format!("Force include pattern: {}", e),
                        })?,
                )
            } else {
                None
            }
        };

        Ok(Self {
            gitignore,
            force_include,
            project_root: project_root.to_path_buf(),
        })
    }

    fn add_default_patterns(builder: &mut GitignoreBuilder) -> Result<()> {
        // Default patterns to ignore (using proper gitignore syntax)
        let default_patterns = [
            ".git",
            ".gitignore",
            ".cpdignore",
            ".cpdforce",
            "target",
            "node_modules",
            ".env",
            ".env.local",
            "*.tmp",
            "*.temp",
            ".DS_Store",
            "Thumbs.db",
            "*.pyc",
            "__pycache__",
            ".pytest_cache",
            ".coverage",
            ".vscode",
            ".idea",
            "*.swp",
            "*.swo",
            "*~",
        ];

        for pattern in &default_patterns {
            builder
                .add_line(None, pattern)
                .map_err(|e| CpdError::InvalidIgnorePattern {
                    pattern: format!("Default pattern '{}': {}", pattern, e),
                })?;
        }

        Ok(())
    }

    /// Check if a file should be included (not ignored)
    pub fn should_include(&self, path: &Path) -> bool {
        // Convert absolute path to relative path from project root
        let relative_path = if path.is_absolute() {
            match path.strip_prefix(&self.project_root) {
                Ok(rel) => rel,
                Err(_) => return true, // If not under project root, include by default
            }
        } else {
            path
        };

        // Check force include patterns first - these override any ignore patterns
        if let Some(ref force_include) = self.force_include {
            match force_include.matched(relative_path, path.is_dir()) {
                Match::None => {} // Not in force include, continue with normal ignore logic
                Match::Ignore(_) => return true, // Force include this file
                Match::Whitelist(_) => return true, // Force include this file
            }
        }

        // Apply normal ignore logic
        match self.gitignore.matched(relative_path, path.is_dir()) {
            Match::None | Match::Whitelist(_) => true,
            Match::Ignore(_) => false,
        }
    }

    /// Get a closure that can be used for filtering
    pub fn filter_fn(&self) -> impl Fn(&Path) -> bool + '_ {
        move |path: &Path| self.should_include(path)
    }

    /// List all patterns that would be applied
    #[allow(dead_code)]
    pub fn list_patterns(&self) -> Vec<String> {
        // This is a simplified version - the actual gitignore crate doesn't expose patterns directly
        // In a real implementation, you might want to store the patterns separately
        vec![
            ".git/".to_string(),
            "target/".to_string(),
            "node_modules/".to_string(),
            "*.pyc".to_string(),
            "__pycache__/".to_string(),
        ]
    }
}

/// Helper function to create a simple filter for testing
#[allow(dead_code)]
pub fn create_simple_filter(patterns: &[&str]) -> Result<impl Fn(&Path) -> bool> {
    use std::path::Path;
    let temp_dir = std::env::temp_dir();
    let mut builder = GitignoreBuilder::new(&temp_dir);

    for pattern in patterns {
        builder
            .add_line(None, pattern)
            .map_err(|e| CpdError::InvalidIgnorePattern {
                pattern: format!("Pattern '{}': {}", pattern, e),
            })?;
    }

    let gitignore = builder
        .build()
        .map_err(|e| CpdError::InvalidIgnorePattern {
            pattern: e.to_string(),
        })?;

    Ok(move |path: &Path| {
        // For testing, treat all paths as relative to avoid path prefix issues
        let relative_path = if path.is_absolute() {
            path.file_name().map(Path::new).unwrap_or(path)
        } else {
            path
        };

        match gitignore.matched(relative_path, false) {
            Match::None | Match::Whitelist(_) => true,
            Match::Ignore(_) => false,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_ignores() {
        let temp_dir = TempDir::new().unwrap();
        let filter = IgnoreFilter::new(temp_dir.path()).unwrap();

        // Test with absolute paths (as they would be used in practice)
        let project_root = temp_dir.path();

        // Should ignore .git directory
        assert!(!filter.should_include(&project_root.join(".git")));
        // Note: gitignore patterns like ".git" only match the exact name, not subdirectories
        // This is expected behavior - to ignore subdirectories, we'd need ".git/**" pattern

        // Should ignore target directory
        assert!(!filter.should_include(&project_root.join("target")));
        // Note: target/* files need explicit patterns or walkdir should handle directory exclusion

        // Should include regular Python files
        assert!(filter.should_include(&project_root.join("main.py")));
        assert!(filter.should_include(&project_root.join("code.py")));

        // Should ignore compiled Python files
        assert!(!filter.should_include(&project_root.join("test.pyc")));
        assert!(!filter.should_include(&project_root.join("__pycache__")));
    }

    #[test]
    fn test_custom_cpdignore() {
        let temp_dir = TempDir::new().unwrap();
        let cpdignore_path = temp_dir.path().join(".cpdignore");

        // Create a .cpdignore file
        fs::write(&cpdignore_path, "custom_ignore\n*.log\ntemp_*").unwrap();

        let filter = IgnoreFilter::new(temp_dir.path()).unwrap();
        let project_root = temp_dir.path();

        // Should ignore custom patterns
        assert!(!filter.should_include(&project_root.join("custom_ignore")));
        assert!(!filter.should_include(&project_root.join("debug.log")));
        assert!(!filter.should_include(&project_root.join("temp_file.txt")));

        // Should still include regular files
        assert!(filter.should_include(&project_root.join("main.py")));
    }

    #[test]
    fn test_simple_filter() {
        let filter = create_simple_filter(&["*.txt", "temp"]).unwrap();

        assert!(!filter(&PathBuf::from("readme.txt")));
        assert!(!filter(&PathBuf::from("temp")));
        assert!(filter(&PathBuf::from("main.py")));
    }

    #[test]
    fn test_cpdforce_functionality() {
        let temp_dir = TempDir::new().unwrap();
        let cpdignore_path = temp_dir.path().join(".cpdignore");
        let cpdforce_path = temp_dir.path().join(".cpdforce");

        // Create a .cpdignore file that ignores .env files
        fs::write(&cpdignore_path, "*.env\nsecrets.txt").unwrap();

        // Create a .cpdforce file that force includes some .env files
        fs::write(&cpdforce_path, "settings.env\nconfig.env").unwrap();

        let filter = IgnoreFilter::new(temp_dir.path()).unwrap();
        let project_root = temp_dir.path();

        // Should ignore .env files by default
        assert!(!filter.should_include(&project_root.join("test.env")));
        assert!(!filter.should_include(&project_root.join("secrets.txt")));

        // Should force include specific .env files
        assert!(filter.should_include(&project_root.join("settings.env")));
        assert!(filter.should_include(&project_root.join("config.env")));

        // Should still include regular files
        assert!(filter.should_include(&project_root.join("main.py")));
    }
}
