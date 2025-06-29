use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpdError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Board not found. Please ensure your CircuitPython board is connected and mounted.")]
    BoardNotFound,

    #[error("Multiple boards detected. Please specify which board to use with --board option.")]
    MultipleBoardsFound,

    #[error("Invalid board path: {path}")]
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
    PermissionDenied { path: String },

    #[error("Disk full or insufficient space")]
    InsufficientSpace,

    #[error("Deployment was cancelled by user")]
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
