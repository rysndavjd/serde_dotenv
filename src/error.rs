#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    UnsupportedDeserialization,
    EmptyString,
    MissingEqualSeparator,
    EmptyIdentifier,
    IdentifierStartsWithDigit,
    InvalidIdentifier { char: char, index: usize },
    ValueUnterminatedSingleQuote { index: usize },
    ValueUnterminatedDoubleQuote { index: usize },
    ValueUnescapedShellChar { char: char, index: usize },
    ValueDanglingEscape,
    ValueNotBoolean,
    ValueNotI8,
    ValueNotI16,
    ValueNotI32,
    ValueNotI64,
    ValueNotU8,
    ValueNotU16,
    ValueNotU32,
    ValueNotU64,
    ValueNotF32,
    ValueNotF64,
    ValueNotUnit,
    Custom(String),
}

impl crate::std::fmt::Display for Error {
    fn fmt(&self, f: &mut crate::std::fmt::Formatter<'_>) -> crate::std::fmt::Result {
        match self {
            Error::UnsupportedDeserialization => {
                write!(f, "Unsupported deserialization method")
            }
            Error::EmptyString => write!(f, "string is empty"),
            Error::MissingEqualSeparator => {
                write!(f, "missing separator equal sign ")
            }
            Error::EmptyIdentifier => write!(f, "identifier is empty"),
            Error::IdentifierStartsWithDigit => write!(f, "identifier starts with a digit"),
            Error::InvalidIdentifier { char, index } => {
                write!(
                    f,
                    "identifier contains invalid character, char: {char:?}, index: {index}"
                )
            }
            Error::ValueUnterminatedSingleQuote { index } => {
                write!(f, "value has unterminated single quote, index: {index}")
            }
            Error::ValueUnterminatedDoubleQuote { index } => {
                write!(f, "value has unterminated double quote, index: {index}")
            }
            Error::ValueUnescapedShellChar { char, index } => {
                write!(
                    f,
                    "value contains unescaped shell character, char: {char:?}, index: {index}"
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
            Error::ValueNotUnit => {
                write!(f, "value is not a unit")
            }
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: alloc::fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }
}

impl serde::de::Error for Error {
    fn custom<T: alloc::fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }
}

impl crate::std::error::Error for Error {}
