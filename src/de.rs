use crate::{error::Error, std::fmt};
use serde::de::{
    Deserialize, Deserializer, Error as DeError, IgnoredAny, MapAccess, SeqAccess, Unexpected,
    Visitor,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
enum QuoteState {
    #[default]
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Var {
    identifier: String,
    value: String,
}

impl Var {
    fn parse_str(v: &str) -> Result<Var, Error> {
        if v.is_empty() {
            return Err(Error::EmptyString);
        }

        let (ident, val) = v.split_once('=').ok_or(Error::MissingEqualSeparator)?;

        if ident.is_empty() {
            return Err(Error::EmptyIdentifier);
        }

        if ident.starts_with(|c: char| c.is_ascii_digit()) {
            return Err(Error::IdentifierStartsWithDigit);
        }

        for (i, c) in ident.char_indices() {
            if !(c.is_ascii_alphabetic() || c == '_') {
                return Err(Error::InvalidIdentifier { char: c, index: i });
            }
        }

        let mut value: String = String::new();

        let mut quote = QuoteState::Unquoted;
        let mut quote_pos = 0usize;
        let mut escaped = false;

        // let mut chars = val.char_indices().peekable();

        for (i, c) in val.char_indices() {
            if c == '\\' && quote != QuoteState::SingleQuoted {
                // println!("top: {i}, {c}");
                // match chars.next() {
                //     Some((i, escaped_char)) => {
                //         // if !matches!(
                //         //     (quote, escaped_char),
                //         //     (QuoteState::SingleQuoted, '\'')
                //         //         | (QuoteState::DoubleQuoted, '"')
                //         //         | (QuoteState::Unquoted, '\'' | '"')
                //         // ) {
                //         //
                //         // }

                //         println!("{i}, {escaped_char}");
                //         value.push(escaped_char);
                //         continue;
                //     }
                //     None => return Err(Error::ValueDanglingEscape),
                // }
                escaped = true;
                continue;
            }

            match quote {
                QuoteState::Unquoted => match c {
                    '\'' => {
                        quote_pos = i;
                        quote = QuoteState::SingleQuoted;
                        continue;
                    }
                    '"' => {
                        quote_pos = i;
                        quote = QuoteState::DoubleQuoted;
                        continue;
                    }
                    '|' | '&' | ';' | '<' | '>' | '(' | ')' | '`' | '\\' => {
                        return Err(Error::ValueUnescapedShellChar { char: c, index: i });
                    }
                    _ => {}
                },
                QuoteState::SingleQuoted if c == '\'' => {
                    debug_assert!(i > quote_pos);

                    quote = QuoteState::Unquoted;
                    quote_pos = 0;
                    continue;
                }
                QuoteState::DoubleQuoted if c == '"' => {
                    debug_assert!(i > quote_pos);

                    quote = QuoteState::Unquoted;
                    quote_pos = 0;
                    continue;
                }
                _ => (),
            }

            value.push(c);
        }

        match quote {
            QuoteState::SingleQuoted => {
                return Err(Error::ValueUnterminatedSingleQuote {
                    index: (ident.len() + 1) + quote_pos,
                });
            }
            QuoteState::DoubleQuoted => {
                return Err(Error::ValueUnterminatedDoubleQuote {
                    index: (ident.len() + 1) + quote_pos,
                });
            }
            _ => (),
        }

        Ok(Var {
            identifier: ident.to_string(),
            value,
        })
    }
}

impl<'de> Deserialize<'de> for Var {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(VarVisitor)
    }
}

struct VarVisitor;

impl<'de> Visitor<'de> for VarVisitor {
    type Value = Var;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a variable as [identifier, value] array, {{identifier, value}} map or \"identifier=value\" string"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_str() -> Result<(), Error> {
        assert_eq!(Var::parse_str(""), Err(Error::EmptyString));
        assert_eq!(
            Var::parse_str("just some string"),
            Err(Error::MissingEqualSeparator)
        );
        assert_eq!(Var::parse_str("=value"), Err(Error::EmptyIdentifier));
        assert_eq!(
            Var::parse_str("1=value"),
            Err(Error::IdentifierStartsWithDigit)
        );
        assert_eq!(
            Var::parse_str("❌=❓"),
            Err(Error::InvalidIdentifier {
                char: '❌',
                index: 0
            })
        );
        assert_eq!(
            Var::parse_str("Identifier=Hello\\"),
            Err(Error::ValueDanglingEscape)
        );

        assert_eq!(
            Var::parse_str("value=\'Hello World"),
            Err(Error::ValueUnterminatedSingleQuote { index: 6 })
        );
        assert_eq!(
            Var::parse_str("Identifier=Hello World\'"),
            Err(Error::ValueUnterminatedSingleQuote { index: 22 })
        );
        assert_eq!(
            Var::parse_str("Identifier=\'Hello\' \'World"),
            Err(Error::ValueUnterminatedSingleQuote { index: 19 })
        );

        assert_eq!(
            Var::parse_str("Identifier=\"Hello World"),
            Err(Error::ValueUnterminatedDoubleQuote { index: 11 })
        );
        assert_eq!(
            Var::parse_str("Identifier=Hello World\""),
            Err(Error::ValueUnterminatedDoubleQuote { index: 22 })
        );
        assert_eq!(
            Var::parse_str("Identifier=\"Hello\" \"World"),
            Err(Error::ValueUnterminatedDoubleQuote { index: 19 })
        );

        // // assert_eq!(
        // //     Var::parse_str("Bell=\x07"),
        // //     Err(Error::ControlChar {
        // //         char: '\x07',
        // //         index: 0
        // //     })
        // // );
        // assert_eq!(Var::parse_str(""), Err(Error::EmptyString));
        // assert_eq!(Var::parse_str(""), Err(Error::EmptyString));
        // assert_eq!(Var::parse_str(""), Err(Error::EmptyString));
        // assert_eq!(Var::parse_str(""), Err(Error::EmptyString));

        let t = Var::parse_str(r#"test="1\" \"2""#).unwrap();

        println!("{:?}", t);

        return Ok(());
    }
}
