use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Invalid JSON syntax: {source}")]
    InvalidJson {
        #[from]
        source: serde_json::Error,
    },

    #[error("Patch failed: {source}")]
    PatchFailed {
        #[from]
        source: json_patch::PatchError,
    },

    #[error("Validation failed: {field} - {message}")]
    ValidationFailed { field: String, message: String },

    #[error("IO error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Backup operation failed: {message}")]
    BackupFailed { message: String },

    #[error("Restore operation failed: {message}")]
    RestoreFailed { message: String },
}

pub type ConfigResult<T> = Result<T, ConfigError>;
