use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Data corruption: {0}")]
    Serialization(String),

    #[error("Unknown error occurred")]
    Unknown,
}

impl AppError {
    pub fn to_user_message(&self) -> String {
        match self {
            AppError::Network(_) => "Please check your internet connection and try again.".to_string(),
            AppError::Validation(msg) => msg.clone(),
            AppError::Internal(_) => "Something went wrong on our end. Please try again later.".to_string(),
            AppError::Serialization(_) => "We encountered an issue processing the data.".to_string(),
            AppError::Unknown => "An unexpected error occurred.".to_string(),
        }
    }
}
