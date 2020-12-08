//! Src: https://github.com/m-rutter/advent-of-code/blob/master/src/error.rs
//! Computer error module
#![allow(dead_code)]
use std::error::Error as StdError;
use std::fmt;

// Convenience Result type
pub type ComputerResult<T> = std::result::Result<T, ComputerError>;

/// An error type for the Advent of Code crate
#[derive(Debug)]
pub struct ComputerError {
    kind: ErrorKind,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl ComputerError {
    pub fn new(msg: String) -> ComputerError {
        Self {
            kind: ErrorKind::Msg(msg),
            source: None,
        }
    }
    ///Creates generic error with a message and a cause
    pub(crate) fn chain(
        value: &impl ToString,
        cause: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: Some(cause.into()),
        }
    }
    pub fn from_option<T>(option: Option<T>) -> ComputerResult<T> {
        if let Some(val) = option {
            Ok(val)
        } else {
            Err(ComputerError::from(ErrorKind::OutOfBoundsError))
        }
    }
}

impl StdError for ComputerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|c| c.as_ref() as &(dyn StdError + 'static))
    }

    fn cause(&self) -> Option<&(dyn StdError)> {
        self.source().as_ref().map(|c| &**c)
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Generic error message
    Msg(String),
    /// Error when parsing provided input
    InputParse(String),
    /// Error when executing code
    ProcessingError,
    /// Out of bounds error
    OutOfBoundsError,
}

impl From<ErrorKind> for ComputerError {
    fn from(error: ErrorKind) -> Self {
        Self {
            kind: error,
            source: None,
        }
    }
}

impl From<std::num::ParseIntError> for ComputerError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self {
            kind: ErrorKind::InputParse(String::from("Couldn't parse integer")),
            source: Some(error.into()),
        }
    }
}
impl From<std::io::Error> for ComputerError {
    fn from(error: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::ProcessingError,
            source: Some(error.into()),
        }
    }
}
impl From<regex::Error> for ComputerError {
    fn from(error: regex::Error) -> Self {
        Self {
            kind: ErrorKind::InputParse(String::from("Couldn't parse regex")),
            source: Some(error.into()),
        }
    }
}
impl fmt::Display for ComputerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::Msg(message) => write!(f, "{}", message),
            ErrorKind::InputParse(message) => write!(f, "Error parsing input: {}", message),
            ErrorKind::ProcessingError => write!(f, "Error when executing code"),
            ErrorKind::OutOfBoundsError => write!(f, "Tried to access memory out of bounds!"),
        }
    }
}
