use alloc::{fmt, format, string::String};

/// Errors that can occur while serializing data.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SerError {
    /// Serialization method used is not supported.
    UnsupportedSerialization,
    /// Key was empty.
    EmptyKey,
    /// Float was not finite (`NaN` or infinite).
    FloatNotFinite,
    /// Key started with an ASCII digit.
    KeyStartsWithDigit,
    /// Key contained a character outside `[A-Za-z0-9_]`.
    InvalidKey,
    /// String contained a null byte (`\0`).
    NullByte,
    /// Value contains invalid UTF-8.
    InvalidUtf8,
    /// The underlying writer returned an IO error.
    IoError,
    /// Custom message produced through [`serde::ser::Error::custom`].
    Custom(String),
}

impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerError::UnsupportedSerialization => {
                write!(f, "serialization method used is not supported")
            }
            SerError::EmptyKey => write!(f, "key was empty"),
            SerError::FloatNotFinite => {
                write!(f, "float was not finite (`NaN` or infinite)")
            }
            SerError::KeyStartsWithDigit => write!(f, "key started with an ASCII digit"),
            SerError::InvalidKey => {
                write!(f, "key contained a character outside `[A-Za-z0-9_]`")
            }
            SerError::NullByte => {
                write!(f, "string contained a null byte (`\0`)")
            }
            SerError::InvalidUtf8 => {
                write!(f, "value contains invalid UTF-8")
            }
            SerError::IoError => {
                write!(f, "the underlying writer returned an IO error")
            }
            SerError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl serde::ser::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerError::Custom(format!("{}", msg))
    }
}

impl serde::de::Error for SerError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        SerError::Custom(format!("{}", msg))
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for SerError {
    fn from(_: std::io::Error) -> Self {
        SerError::IoError
    }
}

#[cfg(feature = "no_std")]
impl<E: embedded_io::Error> From<E> for SerError {
    fn from(_: E) -> Self {
        SerError::IoError
    }
}

impl crate::std::error::Error for SerError {}
