use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cpd")]
#[command(version)]
#[command(about = "Fast, reliable CircuitPython project deployment")]
#[command(long_about = "A command-line tool for deploying CircuitPython projects from your development environment to CircuitPython boards.

Features:
  • Automatic board detection and smart file filtering
  • .cpdignore/.cpdforce support with gitignore-style patterns  
  • Incremental sync (only copy changed files)
  • Backup functionality with progress tracking
  • Cross-platform support (Windows, macOS, Linux)
  • High-performance deployment with visual feedback

Examples:
  cpd                           Deploy current directory to auto-detected board
  cpd --list-boards            Show all detected CircuitPython boards
  cpd --dry-run                Preview deployment without copying files
  cpd --backup ./backup        Create backup before deployment
  cpd --incremental            Only copy files that have changed
  cpd --board /media/CIRCUITPY  Deploy to specific board path")]
pub struct Cli {
    /// Path to the project directory to deploy (defaults to current directory)
    #[arg(value_name = "PROJECT_DIR")]
    pub project_dir: Option<PathBuf>,

    /// Specify the board drive/mount point manually (e.g., E:\, /media/CIRCUITPY)
    #[arg(short = 'b', long = "board", value_name = "BOARD_PATH")]
    pub board_path: Option<PathBuf>,

    /// Backup existing board files before deployment
    #[arg(short = 'B', long = "backup", value_name = "BACKUP_DIR")]
    pub backup_dir: Option<PathBuf>,

    /// Preview deployment without copying files (safe mode)
    #[arg(short = 'n', long = "dry-run")]
    pub dry_run: bool,

    /// Show detailed information during deployment
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Force deployment even if board validation fails
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Skip interactive confirmation prompts
    #[arg(short = 'y', long = "yes")]
    pub assume_yes: bool,

    /// List all detected CircuitPython boards and exit
    #[arg(short = 'l', long = "list-boards")]
    pub list_boards: bool,
    
    /// Only copy changed files (incremental sync)
    #[arg(short = 'i', long = "incremental")]
    pub incremental: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn project_dir(&self) -> PathBuf {
        self.project_dir
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        use crate::error::CpdError;

        // Validate project directory exists
        let project_dir = self.project_dir();
        if !project_dir.exists() {
            return Err(CpdError::Configuration {
                message: format!("Project directory does not exist: {}", project_dir.display()),
            });
        }

        if !project_dir.is_dir() {
            return Err(CpdError::Configuration {
                message: format!("Project path is not a directory: {}", project_dir.display()),
            });
        }

        // Validate board path if specified
        if let Some(board_path) = &self.board_path {
            if !board_path.exists() {
                return Err(CpdError::InvalidBoardPath {
                    path: board_path.display().to_string(),
                });
            }

            if !board_path.is_dir() {
                return Err(CpdError::InvalidBoardPath {
                    path: format!("{} is not a directory", board_path.display()),
                });
            }
        }

        // Validate backup directory if specified
        if let Some(backup_dir) = &self.backup_dir {
            if backup_dir.exists() && !backup_dir.is_dir() {
                return Err(CpdError::Configuration {
                    message: format!("Backup path exists but is not a directory: {}", backup_dir.display()),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_project_dir() {
        let cli = Cli {
            project_dir: None,
            board_path: None,
            backup_dir: None,
            dry_run: false,
            verbose: false,
            force: false,
            assume_yes: false,
            list_boards: false,
            incremental: false,
        };

        let current_dir = env::current_dir().unwrap();
        assert_eq!(cli.project_dir(), current_dir);
    }

    #[test]
    fn test_explicit_project_dir() {
        let test_path = PathBuf::from("/test/path");
        let cli = Cli {
            project_dir: Some(test_path.clone()),
            board_path: None,
            backup_dir: None,
            dry_run: false,
            verbose: false,
            force: false,
            assume_yes: false,
            list_boards: false,
            incremental: false,
        };

        assert_eq!(cli.project_dir(), test_path);
    }
}
