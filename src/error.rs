use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrganizerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, OrganizerError>;
