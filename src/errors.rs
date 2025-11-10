// errors.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedactError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("pattern compile failed: {0}")]
    Pattern(String),

    #[error("invalid span range")]
    BadSpan,
}
