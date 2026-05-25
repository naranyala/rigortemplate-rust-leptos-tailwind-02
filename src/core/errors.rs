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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_user_message_network() {
        let err = AppError::Network("timeout".to_string());
        assert_eq!(err.to_user_message(), "Please check your internet connection and try again.");
    }

    #[test]
    fn test_to_user_message_validation() {
        let err = AppError::Validation("Invalid email".to_string());
        assert_eq!(err.to_user_message(), "Invalid email");
    }

    #[test]
    fn test_to_user_message_internal() {
        let err = AppError::Internal("db crash".to_string());
        assert_eq!(err.to_user_message(), "Something went wrong on our end. Please try again later.");
    }

    #[test]
    fn test_to_user_message_serialization() {
        let err = AppError::Serialization("bad json".to_string());
        assert_eq!(err.to_user_message(), "We encountered an issue processing the data.");
    }

    #[test]
    fn test_to_user_message_unknown() {
        let err = AppError::Unknown;
        assert_eq!(err.to_user_message(), "An unexpected error occurred.");
    }
}
