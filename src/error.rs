use thiserror::Error;

/// Error types for the project.
#[derive(Error, Debug)]
pub enum Error {
    /// General error type. Please prevent using this, instead you should create new error code
    /// below. May be removed in later stages.
    #[error("General error: {0}")]
    General(String),

    /// All IO errors will be wrapped with this.
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// All reqest errors will be wrapped with this.
    #[error(transparent)]
    Network(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;


