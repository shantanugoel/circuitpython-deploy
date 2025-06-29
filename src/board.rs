use crate::error::{CpdError, Result};
use std::path::{Path, PathBuf};
use sysinfo::Disks;

#[derive(Debug, Clone)]
pub struct CircuitPythonBoard {
    pub path: PathBuf,
    pub name: String,
    pub volume_label: Option<String>,
    pub total_space: u64,
    pub available_space: u64,
}

impl CircuitPythonBoard {
    pub fn new(path: PathBuf, name: String, volume_label: Option<String>, total_space: u64, available_space: u64) -> Self {
        Self {
            path,
            name,
            volume_label,
            total_space,
            available_space,
        }
    }
    
    pub fn display_name(&self) -> String {
        match &self.volume_label {
            Some(label) => format!("{} ({})", self.name, label),
            None => self.name.clone(),
        }
    }
    
    pub fn format_space(&self) -> String {
        format!(
            "{} / {} available",
            format_bytes(self.available_space),
            format_bytes(self.total_space)
        )
    }
}

pub struct BoardDetector {
    verbose: bool,
}

impl BoardDetector {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }
    
    /// Detect all available CircuitPython boards
    pub fn detect_boards(&self) -> Result<Vec<CircuitPythonBoard>> {
        let mut boards = Vec::new();
        let disks = Disks::new_with_refreshed_list();
        
        for disk in &disks {
            let mount_point = disk.mount_point();
            
            if self.verbose {
                println!("Checking disk: {} at {}", disk.name().to_string_lossy(), mount_point.display());
            }
            
            if self.is_circuitpython_board(mount_point) {
                let volume_label = self.get_volume_label(mount_point);
                let board = CircuitPythonBoard::new(
                    mount_point.to_path_buf(),
                    disk.name().to_string_lossy().to_string(),
                    volume_label,
                    disk.total_space(),
                    disk.available_space(),
                );
                
                if self.verbose {
                    println!("Found CircuitPython board: {}", board.display_name());
                }
                
                boards.push(board);
            }
        }
        
        Ok(boards)
    }
    
    /// Check if a path represents a CircuitPython board
    pub fn is_circuitpython_board(&self, path: &Path) -> bool {
        if !path.exists() || !path.is_dir() {
            return false;
        }
        
        // Check for common CircuitPython indicators
        let indicators = [
            "boot_out.txt",
            "code.py",
            "main.py",
            "lib",
        ];
        
        let mut found_indicators = 0;
        for indicator in &indicators {
            let indicator_path = path.join(indicator);
            if indicator_path.exists() {
                found_indicators += 1;
            }
        }
        
        // Check for volume label (more reliable but not always available)
        let volume_label = self.get_volume_label(path);
        let has_circuitpy_label = volume_label
            .as_ref()
            .map(|label| label.to_uppercase().contains("CIRCUITPY"))
            .unwrap_or(false);
        
        // Consider it a CircuitPython board if:
        // 1. Has CIRCUITPY volume label, OR
        // 2. Has at least 2 CircuitPython indicators
        has_circuitpy_label || found_indicators >= 2
    }
    
    /// Get the volume label for a mount point
    fn get_volume_label(&self, path: &Path) -> Option<String> {
        // This is a simplified implementation
        // In a real implementation, you'd use platform-specific APIs
        
        #[cfg(windows)]
        {
            self.get_windows_volume_label(path)
        }
        
        #[cfg(unix)]
        {
            self.get_unix_volume_label(path)
        }
    }
    
    #[cfg(windows)]
    fn get_windows_volume_label(&self, path: &Path) -> Option<String> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use winapi::um::fileapi::GetVolumeInformationW;
        
        let path_wide: Vec<u16> = OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        let mut volume_name = [0u16; 256];
        let mut file_system_name = [0u16; 256];
        let mut volume_serial_number = 0;
        let mut maximum_component_length = 0;
        let mut file_system_flags = 0;
        
        unsafe {
            let result = GetVolumeInformationW(
                path_wide.as_ptr(),
                volume_name.as_mut_ptr(),
                volume_name.len() as u32,
                &mut volume_serial_number,
                &mut maximum_component_length,
                &mut file_system_flags,
                file_system_name.as_mut_ptr(),
                file_system_name.len() as u32,
            );
            
            if result != 0 {
                let len = volume_name.iter().position(|&x| x == 0).unwrap_or(volume_name.len());
                if len > 0 {
                    return String::from_utf16(&volume_name[..len]).ok();
                }
            }
        }
        
        None
    }
    
    #[cfg(unix)]
    fn get_unix_volume_label(&self, _path: &Path) -> Option<String> {
        // On Unix systems, we can try to read from /proc/mounts or use system commands
        // For now, return None as a placeholder
        None
    }
    
    /// Interactive board selection
    pub fn select_board<'a>(&self, boards: &'a [CircuitPythonBoard]) -> Result<&'a CircuitPythonBoard> {
        if boards.is_empty() {
            return Err(CpdError::BoardNotFound);
        }
        
        if boards.len() == 1 {
            return Ok(&boards[0]);
        }
        
        println!("Multiple CircuitPython boards detected:");
        for (i, board) in boards.iter().enumerate() {
            println!("  {}: {} at {} ({})", 
                i + 1, 
                board.display_name(), 
                board.path.display(),
                board.format_space()
            );
        }
        
        println!("Please select a board (1-{}):", boards.len());
        
        use std::io::{self, Write};
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim().parse::<usize>() {
                Ok(choice) if choice >= 1 && choice <= boards.len() => {
                    return Ok(&boards[choice - 1]);
                }
                _ => {
                    println!("Invalid selection. Please enter a number between 1 and {}.", boards.len());
                }
            }
        }
    }
    
    /// List all detected boards
    pub fn list_boards(&self) -> Result<()> {
        let boards = self.detect_boards()?;
        
        if boards.is_empty() {
            println!("No CircuitPython boards detected.");
            println!("\nTroubleshooting:");
            println!("  - Ensure your CircuitPython board is connected via USB");
            println!("  - Check that the board appears as a removable drive");
            println!("  - Try pressing the RESET button on your board");
            return Ok(());
        }
        
        println!("Detected CircuitPython boards:");
        for board in &boards {
            println!("  • {} at {}", board.display_name(), board.path.display());
            println!("    Space: {}", board.format_space());
            
            // Show some board details
            if let Ok(entries) = std::fs::read_dir(&board.path) {
                let file_count = entries.count();
                println!("    Files: {} items", file_count);
            }
            
            println!();
        }
        
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[test]
    fn test_is_circuitpython_board() {
        let temp_dir = TempDir::new().unwrap();
        let detector = BoardDetector::new(false);
        
        // Empty directory should not be detected as a board
        assert!(!detector.is_circuitpython_board(temp_dir.path()));
        
        // Directory with boot_out.txt should be detected
        fs::write(temp_dir.path().join("boot_out.txt"), "CircuitPython test").unwrap();
        fs::write(temp_dir.path().join("code.py"), "print('hello')").unwrap();
        
        assert!(detector.is_circuitpython_board(temp_dir.path()));
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
    }
}
