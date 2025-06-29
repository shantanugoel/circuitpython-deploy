use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpdError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("No CircuitPython boards detected.\n\nTroubleshooting:\n  • Ensure your CircuitPython board is connected via USB\n  • Check that the board appears as a removable drive\n  • Try pressing the RESET button on your board\n  • Use --board <path> to specify the board manually")]
    BoardNotFound,

    #[error("Multiple CircuitPython boards detected.\n\nPlease specify which board to use:\n  • Use --board <path> to specify manually, or\n  • Run without --yes flag for interactive selection")]
    MultipleBoardsFound,

    #[error("Invalid board path: {path}\n\nThe specified path does not exist or is not a valid CircuitPython board.\nUse --list-boards to see detected boards.")]
    InvalidBoardPath { path: String },

    #[error("Backup directory creation failed: {path}")]
    BackupDirectoryCreationFailed { path: String },

    #[error("File copy failed: {from} -> {to}")]
    FileCopyFailed { from: String, to: String },

    #[error("Invalid ignore pattern: {pattern}")]
    InvalidIgnorePattern { pattern: String },

    #[error("Configuration error: {message}")]
    Configuration { message: String },

    #[error("Permission denied: {path}")]
    #[allow(dead_code)]
    PermissionDenied { path: String },

    #[error("Disk full or insufficient space")]
    #[allow(dead_code)]
    InsufficientSpace,

    #[error("Deployment was cancelled by user")]
    #[allow(dead_code)]
    Cancelled,
}

pub type Result<T> = std::result::Result<T, CpdError>;

impl CpdError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            CpdError::Io(_) => false,
            CpdError::BoardNotFound => false,
            CpdError::MultipleBoardsFound => false,
            CpdError::InvalidBoardPath { .. } => false,
            CpdError::BackupDirectoryCreationFailed { .. } => false,
            CpdError::FileCopyFailed { .. } => true, // Can continue with other files
            CpdError::InvalidIgnorePattern { .. } => true,
            CpdError::Configuration { .. } => false,
            CpdError::PermissionDenied { .. } => true,
            CpdError::InsufficientSpace => false,
            CpdError::Cancelled => false,
        }
    }
}
