use parside::error::ParsideError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CesrError {
    #[error("Invalid library state")]
    InvalidState,
    #[error("Unexpected variant: {0}")]
    Unexpected(String),
    #[error("Parser error occurred: {0}")]
    ParserError(String),
    #[error("Common error")]
    Common(String),
}

impl From<ParsideError> for CesrError {
    fn from(err: ParsideError) -> CesrError {
        CesrError::ParserError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for CesrError {
    fn from(err: Box<dyn std::error::Error>) -> CesrError {
        CesrError::Common(err.to_string())
    }
}

pub type CesrResult<T> = std::result::Result<T, CesrError>;
