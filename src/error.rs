use thiserror::Error;

/// Custom Result type for the Matar utility.
pub type MatarResult<T> = std::result::Result<T, MatarError>;

#[derive(Error, Debug)]
pub enum MatarError {
    /// Errors occurring when a system command (pgrep, ps) fails to launch or exits with error.
    #[error("Failed to execute system command '{0}': {1}")]
    CommandExecution(String, String),

    /// Standard I/O errors (e.g., reading from /proc).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Didactic Note: This specific error represents a failure to read metadata (path/cmdline)
    /// for a specific PID, usually due to a race condition or permission issues.
    #[error("Could not retrieve metadata for PID {0}: {1}")]
    ProcessMetadata(u32, String),
}
