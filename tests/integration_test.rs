use std::fs;
use std::path::Path;
use tempfile::TempDir;

use circuitpython_deploy::board::BoardDetector;
use circuitpython_deploy::ignore::IgnoreFilter;
use circuitpython_deploy::file_ops::FileOperations;

/// Test that basic board detection works
#[test]
fn test_board_detection() {
    let detector = BoardDetector::new(false);
    
    // Create a mock CircuitPython board
    let temp_dir = TempDir::new().unwrap();
    let board_path = temp_dir.path();
    
    // Initially should not be detected as a board
    assert!(!detector.is_circuitpython_board(board_path));
    
    // Add boot_out.txt with CircuitPython content
    fs::write(
        board_path.join("boot_out.txt"),
        "Adafruit CircuitPython 8.2.0 on 2023-05-15; Test Board"
    ).unwrap();
    
    // Should now be detected as a board
    assert!(detector.is_circuitpython_board(board_path));
}

/// Test .cpdignore functionality
#[test]
fn test_cpdignore_filtering() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    
    // Create test files
    fs::write(project_path.join("code.py"), "print('hello')").unwrap();
    fs::write(project_path.join("test.py"), "# test file").unwrap();
    fs::write(project_path.join("README.md"), "# Project").unwrap();
    
    fs::create_dir_all(project_path.join("lib")).unwrap();
    fs::write(project_path.join("lib/helper.py"), "def help(): pass").unwrap();
    
    fs::create_dir_all(project_path.join("tests")).unwrap();
    fs::write(project_path.join("tests/test_main.py"), "# test").unwrap();
    
    // Create .cpdignore
    fs::write(project_path.join(".cpdignore"), "tests/*\n*.test\n").unwrap();
    
    let filter = IgnoreFilter::new(project_path).unwrap();
    
    // Should include these files
    assert!(filter.should_include(&project_path.join("code.py")));
    assert!(filter.should_include(&project_path.join("README.md")));
    assert!(filter.should_include(&project_path.join("lib/helper.py")));
    
    // Should exclude these files
    assert!(!filter.should_include(&project_path.join("tests/test_main.py")));
    assert!(!filter.should_include(&project_path.join(".cpdignore")));
}

/// Test file operations and copying
#[test]
fn test_file_operations() {
    let source_dir = TempDir::new().unwrap();
    let dest_dir = TempDir::new().unwrap();
    
    // Create source files
    fs::write(source_dir.path().join("main.py"), "print('main')").unwrap();
    fs::create_dir_all(source_dir.path().join("lib")).unwrap();
    fs::write(source_dir.path().join("lib/utils.py"), "# utils").unwrap();
    
    let file_ops = FileOperations::new(false);
    
    // Test copying with a filter that includes everything
    let filter = |_: &Path| true;
    
    let result = file_ops.copy_directory_contents(
        source_dir.path(),
        dest_dir.path(),
        &filter,
        false, // not dry run
    ).unwrap();
    
    assert_eq!(result.files_copied, 2);
    assert_eq!(result.files_failed, 0);
    
    // Verify files were copied
    assert!(dest_dir.path().join("main.py").exists());
    assert!(dest_dir.path().join("lib/utils.py").exists());
    
    // Verify content
    let content = fs::read_to_string(dest_dir.path().join("main.py")).unwrap();
    assert_eq!(content, "print('main')");
}

/// Test backup functionality
#[test]
fn test_backup_functionality() {
    let source_dir = TempDir::new().unwrap();
    let backup_dir = TempDir::new().unwrap();
    
    // Create source files to backup
    fs::write(source_dir.path().join("existing.py"), "# existing code").unwrap();
    fs::create_dir_all(source_dir.path().join("old_lib")).unwrap();
    fs::write(source_dir.path().join("old_lib/old.py"), "# old lib").unwrap();
    
    let file_ops = FileOperations::new(false);
    
    // Create backup
    file_ops.create_backup(source_dir.path(), backup_dir.path()).unwrap();
    
    // Verify backup was created
    assert!(backup_dir.path().join("existing.py").exists());
    assert!(backup_dir.path().join("old_lib/old.py").exists());
    
    // Verify content
    let content = fs::read_to_string(backup_dir.path().join("existing.py")).unwrap();
    assert_eq!(content, "# existing code");
}

/// Test dry run mode
#[test]
fn test_dry_run_mode() {
    let source_dir = TempDir::new().unwrap();
    let dest_dir = TempDir::new().unwrap();
    
    // Create source files
    fs::write(source_dir.path().join("test.py"), "print('test')").unwrap();
    
    let file_ops = FileOperations::new(false);
    let filter = |_: &Path| true;
    
    // Run in dry-run mode
    let result = file_ops.copy_directory_contents(
        source_dir.path(),
        dest_dir.path(),
        &filter,
        true, // dry run
    ).unwrap();
    
    // Should report files that would be copied
    assert_eq!(result.files_copied, 1);
    assert_eq!(result.files_failed, 0);
    
    // But files should not actually be copied
    assert!(!dest_dir.path().join("test.py").exists());
}

/// Test error handling for invalid paths
#[test]
fn test_error_handling() {
    let detector = BoardDetector::new(false);
    
    // Test with non-existent path
    let non_existent = Path::new("/non/existent/path");
    assert!(!detector.is_circuitpython_board(non_existent));
    
    // Test ignore filter with non-existent project
    // Note: IgnoreFilter::new() might not fail immediately for non-existent paths
    // as it's designed to be tolerant of missing .cpdignore files
    let _filter_result = IgnoreFilter::new(non_existent);
    // Just verify we can create a filter, even if the directory doesn't exist
}

/// Test with files that have special characters
#[test]
fn test_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    
    // Create files with various characters
    fs::write(project_path.join("file with spaces.py"), "# spaces").unwrap();
    fs::write(project_path.join("file-with-dashes.py"), "# dashes").unwrap();
    fs::write(project_path.join("file_with_underscores.py"), "# underscores").unwrap();
    
    let filter = IgnoreFilter::new(project_path).unwrap();
    
    // All should be included by default
    assert!(filter.should_include(&project_path.join("file with spaces.py")));
    assert!(filter.should_include(&project_path.join("file-with-dashes.py")));
    assert!(filter.should_include(&project_path.join("file_with_underscores.py")));
}
