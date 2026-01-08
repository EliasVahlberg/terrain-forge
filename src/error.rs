use std::fmt;

/// Error types for terrain generation
#[derive(Debug)]
pub enum Error {
    /// Grid dimensions are invalid
    InvalidDimensions { width: usize, height: usize },
    /// Algorithm failed to generate valid output
    GenerationFailed(String),
    /// Constraint validation failed
    ConstraintViolation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidDimensions { width, height } => {
                write!(f, "Invalid grid dimensions: {}x{}", width, height)
            }
            Error::GenerationFailed(msg) => write!(f, "Generation failed: {}", msg),
            Error::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type alias for terrain operations
pub type Result<T> = std::result::Result<T, Error>;
