use alloc::{fmt, format, string::String};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    /// Deserialization method used is not supported
    UnsupportedDeserialization,
    /// Serialization method used is not supported
    UnsupportedSerialization,
    /// String is empty
    EmptyString,
    /// Missing `=` sign separator
    MissingEqualSeparator,
    /// Identfifier is empty
    EmptyIdentifier,
    /// Identifier starts with a digit
    IdentifierStartsWithDigit,
    /// Identifier contains a value that is not an ascii alphabetical character or `_`
    InvalidIdentifier {
        index: usize,
    },
    /// Value contains an unterminated `'`
    ValueUnterminatedSingleQuote,
    /// Value contains an unterminated `"`
    ValueUnterminatedDoubleQuote,
    /// Value contains unescaped special shell character (`|`, `&`, `;`, `<`, `>`, `(`, `)`, `` ` ``, `\`, ` `, `\t`)
    ValueUnescapedShellChar {
        index: usize,
    },
    /// Value has a dangling escape character `\`
    ValueDanglingEscape,
    /// Value is not a [`bool`]
    ValueNotBoolean,
    /// Value is not a [`i8`]
    ValueNotI8,
    /// Value is not a [`i16`]
    ValueNotI16,
    /// Value is not a [`i32`]
    ValueNotI32,
    /// Value is not a [`i64`]
    ValueNotI64,
    /// Value is not a [`u8`]
    ValueNotU8,
    /// Value is not a [`u16`]
    ValueNotU16,
    /// Value is not a [`u32`]
    ValueNotU32,
    /// Value is not a [`u64`]
    ValueNotU64,
    /// Value is not a [`f32`]
    ValueNotF32,
    /// Value is not a [`f64`]
    ValueNotF64,
    /// Float given is not finite.
    FloatNotFinite,
    /// Value contains invalid UTF-8.
    InvaildUtf8,
    /// Value is not a [`unit`]
    ValueNotUnit,
    /// An IO Error as occured
    IoError,
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnsupportedDeserialization => {
                write!(f, "Unsupported deserialization method")
            }
            Error::UnsupportedSerialization => {
                write!(f, "Unsupported serialization method")
            }
            Error::EmptyString => write!(f, "string is empty"),
            Error::MissingEqualSeparator => {
                write!(f, "missing separator equal sign ")
            }
            Error::EmptyIdentifier => write!(f, "identifier is empty"),
            Error::IdentifierStartsWithDigit => write!(f, "identifier starts with a digit"),
            Error::InvalidIdentifier { index } => {
                write!(f, "identifier contains invalid character, index: {index}")
            }
            Error::ValueUnterminatedSingleQuote => {
                write!(f, "value has unterminated single quote")
            }
            Error::ValueUnterminatedDoubleQuote => {
                write!(f, "value has unterminated double quote")
            }
            Error::ValueUnescapedShellChar { index } => {
                write!(
                    f,
                    "value contains unescaped shell character, index: {index}"
                )
            }
            Error::ValueDanglingEscape => {
                write!(f, "value has dangling escape character")
            }
            Error::ValueNotBoolean => {
                write!(f, "value is not a boolean")
            }
            Error::ValueNotI8 => {
                write!(f, "value is not a 8-bit signed integer")
            }
            Error::ValueNotI16 => {
                write!(f, "value is not a 16-bit signed integer")
            }
            Error::ValueNotI32 => {
                write!(f, "value is not a 32-bit signed integer")
            }
            Error::ValueNotI64 => {
                write!(f, "value is not a 64-bit signed integer")
            }
            Error::ValueNotU8 => {
                write!(f, "value is not a 8-bit unsigned integer")
            }
            Error::ValueNotU16 => {
                write!(f, "value is not a 16-bit unsigned integer")
            }
            Error::ValueNotU32 => {
                write!(f, "value is not a 32-bit unsigned integer")
            }
            Error::ValueNotU64 => {
                write!(f, "value is not a 64-bit unsigned integer")
            }
            Error::ValueNotF32 => {
                write!(f, "value is not a 32-bit float")
            }
            Error::ValueNotF64 => {
                write!(f, "value is not a 64-bit float")
            }
            Error::FloatNotFinite => {
                write!(f, "float is not a finite number")
            }
            Error::InvaildUtf8 => {
                write!(f, "value contains invalid utf-8")
            }
            Error::ValueNotUnit => {
                write!(f, "value is not a unit")
            }
            Error::IoError => {
                write!(f, "io error")
            }
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IoError
    }
}

#[cfg(feature = "no_std")]
impl<E: embedded_io::Error> From<E> for Error {
    fn from(_: E) -> Self {
        Error::IoError
    }
}

impl crate::std::error::Error for Error {}
