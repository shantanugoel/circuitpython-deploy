use crate::error::{CpdError, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileInfo {
    #[allow(dead_code)]
    pub path: PathBuf,
    pub size: u64,
    pub modified: SystemTime,
}

impl FileInfo {
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path).map_err(|e| CpdError::IoError {
            path: path.to_path_buf(),
            source: e,
        })?;
        
        Ok(Self {
            path: path.to_path_buf(),
            size: metadata.len(),
            modified: metadata.modified().map_err(|e| CpdError::IoError {
                path: path.to_path_buf(),
                source: e,
            })?,
        })
    }
}

/// Compare two files to see if they're different
pub fn files_differ(source_path: &Path, dest_path: &Path) -> Result<bool> {
    // If destination doesn't exist, they differ
    if !dest_path.exists() {
        return Ok(true);
    }
    
    let source_info = FileInfo::from_path(source_path)?;
    let dest_info = FileInfo::from_path(dest_path)?;
    
    // Quick check: if sizes differ, files are different
    if source_info.size != dest_info.size {
        return Ok(true);
    }
    
    // If sizes are the same, check modification times
    // If source is newer than destination, they differ
    Ok(source_info.modified > dest_info.modified)
}

/// Check if any files in the source directory have changed compared to destination
pub fn has_changes(source_dir: &Path, dest_dir: &Path, filter_fn: impl Fn(&Path) -> bool) -> Result<Vec<PathBuf>> {
    let mut changed_files = Vec::new();
    
    for entry in walkdir::WalkDir::new(source_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let source_path = entry.path();
        
        // Apply filter (e.g., ignore patterns)
        if !filter_fn(source_path) {
            continue;
        }
        
        let relative_path = source_path.strip_prefix(source_dir).map_err(|_| CpdError::InvalidPath {
            path: source_path.to_path_buf(),
            reason: "Could not make path relative to source directory".to_string(),
        })?;
        
        let dest_path = dest_dir.join(relative_path);
        
        if files_differ(source_path, &dest_path)? {
            changed_files.push(source_path.to_path_buf());
        }
    }
    
    Ok(changed_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_files_differ_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");
        
        fs::write(&source, "content").unwrap();
        
        // Destination doesn't exist, should differ
        assert!(files_differ(&source, &dest).unwrap());
    }
    
    #[test]
    fn test_files_differ_same_content() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");
        
        fs::write(&source, "content").unwrap();
        fs::write(&dest, "content").unwrap();
        
        // Same content and similar timestamps, should not differ
        assert!(!files_differ(&source, &dest).unwrap());
    }
    
    #[test]
    fn test_files_differ_different_size() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");
        
        fs::write(&source, "longer content").unwrap();
        fs::write(&dest, "short").unwrap();
        
        // Different sizes, should differ
        assert!(files_differ(&source, &dest).unwrap());
    }
    
    #[test]
    fn test_files_differ_newer_source() {
        let temp_dir = TempDir::new().unwrap();
        let source = temp_dir.path().join("source.txt");
        let dest = temp_dir.path().join("dest.txt");
        
        fs::write(&dest, "content").unwrap();
        
        // Wait a bit to ensure different timestamps
        thread::sleep(Duration::from_millis(10));
        
        fs::write(&source, "content").unwrap();
        
        // Source is newer, should differ
        assert!(files_differ(&source, &dest).unwrap());
    }
}
