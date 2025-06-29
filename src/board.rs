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
        
        // Check for volume label first (most reliable)
        let volume_label = self.get_volume_label(path);
        let has_circuitpy_label = volume_label
            .as_ref()
            .map(|label| label.to_uppercase().contains("CIRCUITPY"))
            .unwrap_or(false);
        
        if has_circuitpy_label {
            return true;
        }
        
        // Check for CircuitPython-specific files
        let optional_files = ["code.py", "main.py", "lib"];
        
        // Must have boot_out.txt (created by CircuitPython on boot)
        let has_boot_out = path.join("boot_out.txt").exists();
        if !has_boot_out {
            return false;
        }
        
        // Count optional indicators
        let mut found_optional = 0;
        for file in &optional_files {
            if path.join(file).exists() {
                found_optional += 1;
            }
        }
        
        // Additional check: if boot_out.txt exists, check its content
        if let Ok(content) = std::fs::read_to_string(path.join("boot_out.txt")) {
            // CircuitPython boot_out.txt typically contains "CircuitPython" or "Adafruit"
            let content_lower = content.to_lowercase();
            if content_lower.contains("circuitpython") || content_lower.contains("adafruit") {
                return true;
            }
        }
        
        // Check for other CircuitPython indicators
        let cp_indicators = [
            "CIRCUITPY.USB_VID",
            "CIRCUITPY.USB_PID", 
            "settings.toml",
            ".fseventsd", // macOS creates this on CircuitPython drives
        ];
        
        let mut found_cp_indicators = 0;
        for indicator in &cp_indicators {
            if path.join(indicator).exists() {
                found_cp_indicators += 1;
            }
        }
        
        // Consider it a CircuitPython board if:
        // 1. Has boot_out.txt AND at least one optional file, OR
        // 2. Has boot_out.txt AND at least one CP-specific indicator
        has_boot_out && (found_optional >= 1 || found_cp_indicators >= 1)
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
        
        // Get the root path for the volume (e.g., "C:\\" from "C:\Users\...")
        let root_path = if let Some(root) = path.components().next() {
            let mut root_str = root.as_os_str().to_string_lossy().to_string();
            if !root_str.ends_with('\\') {
                root_str.push('\\');
            }
            root_str
        } else {
            return None;
        };
        
        let path_wide: Vec<u16> = OsStr::new(&root_path)
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
                    let label = String::from_utf16(&volume_name[..len]).ok()?;
                    if !label.trim().is_empty() {
                        return Some(label.trim().to_string());
                    }
                }
            }
        }
        
        // Fallback: try to get volume label from disk info in sysinfo
        self.get_volume_label_from_sysinfo(path)
    }
    
    #[cfg(unix)]
    fn get_unix_volume_label(&self, path: &Path) -> Option<String> {
        // On Unix systems, try multiple approaches to get volume label
        
        // 1. Try to read from mount info
        if let Some(label) = self.get_unix_label_from_mounts(path) {
            return Some(label);
        }
        
        // 2. Try blkid command (if available)
        if let Some(label) = self.get_unix_label_from_blkid(path) {
            return Some(label);
        }
        
        // 3. Fallback to sysinfo
        self.get_volume_label_from_sysinfo(path)
    }
    
    #[cfg(unix)]
    fn get_unix_label_from_mounts(&self, path: &Path) -> Option<String> {
        use std::fs;
        
        // Read /proc/mounts to find the mount point and device
        if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
            for line in mounts.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let mount_point = parts[1];
                    if path.starts_with(mount_point) {
                        // Found the mount point, try to extract label from device name
                        let device = parts[0];
                        if device.contains("CIRCUITPY") {
                            return Some("CIRCUITPY".to_string());
                        }
                    }
                }
            }
        }
        None
    }
    
    #[cfg(unix)]
    fn get_unix_label_from_blkid(&self, _path: &Path) -> Option<String> {
        // This would require executing blkid command
        // For now, we'll skip this to avoid complexity
        None
    }
    
    /// Fallback method to get volume label from sysinfo (works on all platforms)
    fn get_volume_label_from_sysinfo(&self, path: &Path) -> Option<String> {
        let disks = Disks::new_with_refreshed_list();
        
        for disk in &disks {
            if disk.mount_point() == path {
                let name = disk.name().to_string_lossy();
                if !name.is_empty() && name != "Unknown" {
                    return Some(name.to_string());
                }
            }
        }
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
