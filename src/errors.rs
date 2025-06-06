use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    ValidationError(String),
    DatabaseError(String),
    AuthenticationError(String),
    StellarError(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
            AppError::StellarError(msg) => write!(f, "Stellar Error: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
