#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    EmptyString,
    MissingEqualSeparator,
    EmptyIdentifier,
    IdentifierStartsWithDigit,
    InvalidIdentifier { char: char, index: usize },
    ValueUnterminatedSingleQuote { index: usize },
    ValueUnterminatedDoubleQuote { index: usize },
    InvalidValue { char: char, index: usize },
    ValueUnescapedShellChar { char: char, index: usize },
    ValueDanglingEscape,
    Custom(String),
}

impl<'a> crate::std::fmt::Display for Error {
    fn fmt(&self, f: &mut crate::std::fmt::Formatter<'_>) -> crate::std::fmt::Result {
        match self {
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
            Error::InvalidValue { char, index } => {
                write!(
                    f,
                    "value contains invalid character, char: {char:?}, index: {index}"
                )
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
