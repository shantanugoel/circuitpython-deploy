mod cli;
mod error;
mod file_ops;
mod ignore;
mod board;

use cli::Cli;
use error::{CpdError, Result};
use file_ops::FileOperations;
use ignore::IgnoreFilter;
use board::BoardDetector;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse_args();
    
    // Validate CLI arguments
    cli.validate()?;
    
    // Handle list-boards command
    if cli.list_boards {
        let detector = BoardDetector::new(cli.verbose);
        return detector.list_boards();
    }
    
    let project_dir = cli.project_dir();
    
    if cli.verbose {
        println!("Project directory: {}", project_dir.display());
    }
    
    // Set up ignore filter
    let ignore_filter = IgnoreFilter::new(&project_dir)?;
    
    // Detect or validate board
    let detector = BoardDetector::new(cli.verbose);
    let board = if let Some(board_path) = &cli.board_path {
        // Validate manually specified board
        if !detector.is_circuitpython_board(board_path) {
            if cli.force {
                if cli.verbose {
                    println!("Warning: {} doesn't appear to be a CircuitPython board, but --force was specified", board_path.display());
                }
            } else {
                return Err(CpdError::InvalidBoardPath {
                    path: format!("{} doesn't appear to be a CircuitPython board", board_path.display()),
                });
            }
        }
        
        board::CircuitPythonBoard::new(
            board_path.clone(),
            "Manual".to_string(),
            None,
            0,
            0,
        )
    } else {
        // Auto-detect boards
        let boards = detector.detect_boards()?;
        
        if boards.is_empty() {
            return Err(CpdError::BoardNotFound);
        }
        
        if boards.len() == 1 {
            boards.into_iter().next().unwrap()
        } else if cli.assume_yes {
            return Err(CpdError::MultipleBoardsFound);
        } else {
            detector.select_board(&boards)?.clone()
        }
    };
    
    if cli.verbose {
        println!("Target board: {} at {}", board.display_name(), board.path.display());
    }
    
    // Create backup if requested (skip in dry-run mode)
    if let Some(backup_dir) = &cli.backup_dir {
        if cli.dry_run {
            if cli.verbose {
                println!("Would create backup at: {}", backup_dir.display());
            }
        } else {
            if cli.verbose {
                println!("Creating backup at: {}", backup_dir.display());
            }
            
            let file_ops = FileOperations::new(cli.verbose);
            file_ops.create_backup(&board.path, backup_dir)?;
        }
    }
    
    // Show deployment plan
    if cli.verbose || cli.dry_run {
        println!("\nDeployment plan:");
        println!("  Source: {}", project_dir.display());
        println!("  Target: {}", board.path.display());
        
        if let Some(backup_dir) = &cli.backup_dir {
            if cli.dry_run {
                println!("  Backup: Would backup to {}", backup_dir.display());
            } else {
                println!("  Backup: Created at {}", backup_dir.display());
            }
        }
        
        if cli.dry_run {
            println!("  Mode: DRY RUN (no files will be copied)");
        }
        
        println!();
    }
    
    // Confirm deployment unless --yes is specified
    if !cli.assume_yes && !cli.dry_run {
        println!("Deploy to {}? [y/N]", board.display_name());
        
        use std::io::{self, Write};
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Deployment cancelled.");
            return Ok(());
        }
    }
    
    // Perform deployment
    let file_ops = FileOperations::new(cli.verbose);
    let filter_fn = ignore_filter.filter_fn();
    
    let result = file_ops.copy_directory_contents(
        &project_dir,
        &board.path,
        &filter_fn,
        cli.dry_run,
    )?;
    
    // Display results
    println!("\n{}", result.summary());
    
    if result.files_copied == 0 && result.files_failed == 0 {
        println!("\n💡 No files to deploy. This could happen if:");
        println!("   • All files are excluded by .cpdignore patterns");
        println!("   • The project directory is empty");
        println!("   • All files already exist and are unchanged");
        println!("\nTip: Use --verbose --dry-run to see what files would be included.");
    }
    
    if !result.failed_files.is_empty() {
        println!("\n❌ Failed files:");
        for (file, error) in &result.failed_files {
            println!("  {}: {}", file.display(), error);
        }
    }
    
    if !cli.dry_run {
        if result.files_copied > 0 {
            println!("\n✅ Deployment completed successfully!");
            
            if cli.verbose {
                println!("📁 Files deployed:");
                // We could track and show which files were deployed here
                // For now, show general info
            }
            
            // Show board space after deployment
            if let Ok(boards) = detector.detect_boards() {
                if let Some(updated_board) = boards.iter().find(|b| b.path == board.path) {
                    println!("💾 Board space: {}", updated_board.format_space());
                }
            }
            
            println!("\n🚀 Your CircuitPython project is ready to run!");
        }
    } else {
        println!("\n🔍 Dry run completed. Use the command without --dry-run to deploy.");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    
    #[test]
    fn test_basic_deployment_logic() {
        // This test verifies that the main components can be instantiated
        // More comprehensive integration tests would require actual CircuitPython boards
        
        let temp_dir = TempDir::new().unwrap();
        
        // Create a basic project structure
        fs::write(temp_dir.path().join("code.py"), "print('Hello, CircuitPython!')").unwrap();
        fs::create_dir_all(temp_dir.path().join("lib")).unwrap();
        fs::write(temp_dir.path().join("lib/helper.py"), "def help(): pass").unwrap();
        
        // Test ignore filter creation
        let ignore_filter = IgnoreFilter::new(temp_dir.path());
        assert!(ignore_filter.is_ok());
        
        // Test file operations creation
        let file_ops = FileOperations::new(false);
        
        // Test board detector creation
        let detector = BoardDetector::new(false);
        
        // These should not panic and should create valid instances
        drop(file_ops);
        drop(detector);
    }
}
