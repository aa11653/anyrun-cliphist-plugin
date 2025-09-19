use thiserror::Error;

/// A clipboard entry from `cliphist list`.
pub struct ClipboardEntry {
    pub index: u64,
    pub content: String,
}

/// An error type for interacting with `cliphist`.
#[derive(Debug, Error)]
pub enum CliphistError {
    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}
