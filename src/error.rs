use core::fmt;

use serde::de::Error as DeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl DeError for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(format!("{}", msg))
    }
}
