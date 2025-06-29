use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cpd")]
#[command(version = "0.1.0")]
#[command(about = "Deploy CircuitPython projects to boards")]
#[command(long_about = "A command-line tool for deploying CircuitPython projects from local development environment to CircuitPython boards. Supports automatic board detection, .cpdignore files, and backup functionality.")]
pub struct Cli {
    /// Path to the project directory to deploy (defaults to current directory)
    #[arg(value_name = "PROJECT_DIR")]
    pub project_dir: Option<PathBuf>,

    /// Specify the board drive/mount point manually
    #[arg(short = 'b', long = "board", value_name = "BOARD_PATH")]
    pub board_path: Option<PathBuf>,

    /// Backup existing board files to specified directory before deployment
    #[arg(short = 'B', long = "backup", value_name = "BACKUP_DIR")]
    pub backup_dir: Option<PathBuf>,

    /// Show what would be deployed without actually copying files
    #[arg(short = 'n', long = "dry-run")]
    pub dry_run: bool,

    /// Enable verbose output
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Force deployment even if board contains different project
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Skip confirmation prompts
    #[arg(short = 'y', long = "yes")]
    pub assume_yes: bool,

    /// List available CircuitPython boards and exit
    #[arg(short = 'l', long = "list-boards")]
    pub list_boards: bool,
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
        };

        assert_eq!(cli.project_dir(), test_path);
    }
}
