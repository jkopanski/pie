use crate::parser::ParseError;
use miette::Diagnostic;
use rustyline::error::ReadlineError;
use thiserror::Error;
use xdg::BaseDirectoriesError;

#[derive(Debug, Diagnostic, Error)]
pub enum ReadingError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Error while reading line")]
    LineError(#[from] ReadlineError),
    #[error("Couldn't establish pie directories")]
    XdgError(#[from] BaseDirectoriesError),
}

#[derive(Debug, Diagnostic, Error)]
pub enum PieError {
    #[error(transparent)]
    Reading(#[from] ReadingError),
    #[error(transparent)]
    Frontend(#[from] ParseError),
}

impl From<std::io::Error> for PieError {
    fn from(value: std::io::Error) -> Self {
        PieError::from(ReadingError::from(value))
    }
}

impl From<ReadlineError> for PieError {
    fn from(value: ReadlineError) -> Self {
        PieError::from(ReadingError::from(value))
    }
}

impl From<BaseDirectoriesError> for PieError {
    fn from(value: BaseDirectoriesError) -> Self {
        PieError::from(ReadingError::from(value))
    }
}

// shouldn't this be an instance on std::result::Result?
pub fn from_res<T, E1, E2>(value: std::result::Result<T, E1>) -> std::result::Result<T, E2>
where
    E1: Into<E2>,
{
    match value {
        Ok(v) => Ok(v),
        Err(err) => Err(err.into()),
    }
}

pub type Result<T> = std::result::Result<T, PieError>;
