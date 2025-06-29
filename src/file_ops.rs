use crate::error::{CpdError, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileOperations {
    verbose: bool,
}

impl FileOperations {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Copy a single file from source to destination
    pub fn copy_file(&self, from: &Path, to: &Path) -> Result<()> {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                if self.verbose {
                    eprintln!("Failed to create directory {}: {}", parent.display(), e);
                }
                CpdError::Io(e)
            })?;
        }

        fs::copy(from, to).map_err(|e| {
            if self.verbose {
                eprintln!("Failed to copy {} to {}: {}", from.display(), to.display(), e);
            }
            CpdError::FileCopyFailed {
                from: from.display().to_string(),
                to: to.display().to_string(),
            }
        })?;

        // Preserve timestamps
        if let Ok(metadata) = fs::metadata(from) {
            if let Ok(modified) = metadata.modified() {
                let _ = filetime::set_file_mtime(to, filetime::FileTime::from_system_time(modified));
            }
        }

        if self.verbose {
            println!("Copied: {} -> {}", from.display(), to.display());
        }

        Ok(())
    }

    /// Copy directory contents with progress tracking
    pub fn copy_directory_contents(
        &self,
        from_dir: &Path,
        to_dir: &Path,
        filter: &dyn Fn(&Path) -> bool,
        dry_run: bool,
    ) -> Result<CopyResult> {
        let mut files_to_copy = Vec::new();
        let mut _total_size = 0u64;

        // First pass: collect files and calculate total size
        for entry in WalkDir::new(from_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if filter(path) {
                if let Ok(metadata) = entry.metadata() {
                    _total_size += metadata.len();
                }
                files_to_copy.push(path.to_path_buf());
            }
        }

        let progress = if !dry_run && !files_to_copy.is_empty() {
            let pb = ProgressBar::new(files_to_copy.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                    .unwrap()
                    .progress_chars("##-"),
            );
            Some(pb)
        } else {
            None
        };

        let mut result = CopyResult {
            files_copied: 0,
            files_failed: 0,
            bytes_copied: 0,
            failed_files: Vec::new(),
        };

        // Second pass: copy files
        for file_path in &files_to_copy {
            let relative_path = file_path.strip_prefix(from_dir).unwrap();
            let dest_path = to_dir.join(relative_path);

            if let Some(pb) = &progress {
                pb.set_message(format!("Copying {}", relative_path.display()));
            }

            if dry_run {
                println!("Would copy: {} -> {}", file_path.display(), dest_path.display());
                result.files_copied += 1;
            } else {
                match self.copy_file(file_path, &dest_path) {
                    Ok(_) => {
                        result.files_copied += 1;
                        if let Ok(metadata) = fs::metadata(file_path) {
                            result.bytes_copied += metadata.len();
                        }
                    }
                    Err(e) => {
                        result.files_failed += 1;
                        result.failed_files.push((file_path.clone(), e.to_string()));
                        
                        // Continue with other files if error is recoverable
                        if !e.is_recoverable() {
                            if let Some(pb) = &progress {
                                pb.finish_with_message("Deployment failed");
                            }
                            return Err(e);
                        }
                    }
                }
            }

            if let Some(pb) = &progress {
                pb.inc(1);
            }
        }

        if let Some(pb) = &progress {
            pb.finish_with_message("Deployment completed");
        }

        Ok(result)
    }

    /// Create a backup of the destination directory
    pub fn create_backup(&self, source_dir: &Path, backup_dir: &Path) -> Result<()> {
        if !source_dir.exists() {
            return Ok(()); // Nothing to backup
        }

        fs::create_dir_all(backup_dir).map_err(|_| CpdError::BackupDirectoryCreationFailed {
            path: backup_dir.display().to_string(),
        })?;

        let result = self.copy_directory_contents(
            source_dir,
            backup_dir,
            &|_| true, // Backup everything
            false,
        )?;

        if self.verbose {
            println!(
                "Backup completed: {} files, {} bytes",
                result.files_copied,
                format_bytes(result.bytes_copied)
            );
        }

        Ok(())
    }

    /// Remove files that don't exist in source (for clean deployment)
    pub fn clean_destination(&self, source_dir: &Path, dest_dir: &Path, filter: &dyn Fn(&Path) -> bool) -> Result<()> {
        if !dest_dir.exists() {
            return Ok(());
        }

        let mut files_to_remove = Vec::new();

        for entry in WalkDir::new(dest_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let dest_path = entry.path();
            let relative_path = dest_path.strip_prefix(dest_dir).unwrap();
            let source_path = source_dir.join(relative_path);

            // If the file doesn't exist in source or would be filtered out, mark for removal
            if !source_path.exists() || !filter(&source_path) {
                files_to_remove.push(dest_path.to_path_buf());
            }
        }

        for file_path in files_to_remove {
            if let Err(e) = fs::remove_file(&file_path) {
                if self.verbose {
                    eprintln!("Failed to remove {}: {}", file_path.display(), e);
                }
            } else if self.verbose {
                println!("Removed: {}", file_path.display());
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CopyResult {
    pub files_copied: usize,
    pub files_failed: usize,
    pub bytes_copied: u64,
    pub failed_files: Vec<(PathBuf, String)>,
}

impl CopyResult {
    pub fn is_success(&self) -> bool {
        self.files_failed == 0
    }

    pub fn summary(&self) -> String {
        if self.files_failed == 0 {
            format!(
                "Successfully copied {} files ({})",
                self.files_copied,
                format_bytes(self.bytes_copied)
            )
        } else {
            format!(
                "Copied {} files, {} failed ({})",
                self.files_copied,
                self.files_failed,
                format_bytes(self.bytes_copied)
            )
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

// Add filetime dependency to Cargo.toml for timestamp preservation
extern crate filetime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }
}
