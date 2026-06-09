use crate::{common::QuoteState, error::Error, std::mem::take};
use alloc::borrow::Cow;
use serde::de::{self, Deserialize, DeserializeSeed, MapAccess, value::StrDeserializer};

pub fn parse_str<'a>(v: &'a str) -> Result<(&'a str, Cow<'a, str>), Error> {
    if v.is_empty() {
        return Err(Error::EmptyString);
    }

    let (ident, val) = v.split_once('=').ok_or(Error::MissingEqualSeparator)?;

    let mut ident_iter = ident.char_indices();

    match ident_iter.next() {
        Some((_, c)) if !(c.is_ascii_alphabetic() || c == '_') && c.is_ascii_digit() => {
            return Err(Error::IdentifierStartsWithDigit);
        }
        Some((i, c)) if !(c.is_ascii_alphabetic() || c == '_') => {
            return Err(Error::InvalidIdentifier { char: c, index: i });
        }
        None => return Err(Error::EmptyIdentifier),
        _ => (),
    }

    for (i, c) in ident_iter {
        if !(c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::InvalidIdentifier { char: c, index: i });
        }
    }

    if !val.contains(['"', '\'', '\\', '|', '&', ';', '<', '>', '(', ')', '`']) {
        return Ok((ident, Cow::Borrowed(val)));
    }

    let mut output = String::new();

    let mut quote = QuoteState::Unquoted;
    let mut quote_pos = 0usize;

    let mut chars = val.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '\\' && quote != QuoteState::SingleQuoted {
            match chars.next() {
                Some((_, escaped_char)) => {
                    output.push(escaped_char);
                    continue;
                }
                None => return Err(Error::ValueDanglingEscape),
            }
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

        output.push(c);
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

    Ok((ident, Cow::Owned(output)))
}

pub struct ValueDeserializer<'a> {
    raw: Cow<'a, str>,
}

impl<'de> de::Deserializer<'de> for &mut ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let bool = match self.raw.as_ref() {
            "true" => true,
            "false" => false,
            _ => return Err(Error::ValueNotBoolean),
        };

        visitor.visit_bool(bool)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i8 = self.raw.parse::<i8>().map_err(|_| Error::ValueNotI8)?;

        visitor.visit_i8(i8)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i16 = self.raw.parse::<i16>().map_err(|_| Error::ValueNotI16)?;

        visitor.visit_i16(i16)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i32 = self.raw.parse::<i32>().map_err(|_| Error::ValueNotI32)?;

        visitor.visit_i32(i32)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let i64 = self.raw.parse::<i64>().map_err(|_| Error::ValueNotI64)?;

        visitor.visit_i64(i64)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u8 = self.raw.parse::<u8>().map_err(|_| Error::ValueNotU8)?;

        visitor.visit_u8(u8)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u16 = self.raw.parse::<u16>().map_err(|_| Error::ValueNotU16)?;

        visitor.visit_u16(u16)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u32 = self.raw.parse::<u32>().map_err(|_| Error::ValueNotU32)?;

        visitor.visit_u32(u32)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let u64 = self.raw.parse::<u64>().map_err(|_| Error::ValueNotU64)?;

        visitor.visit_u64(u64)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let f32 = self.raw.parse::<f32>().map_err(|_| Error::ValueNotF32)?;

        visitor.visit_f32(f32)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let f64 = self.raw.parse::<f64>().map_err(|_| Error::ValueNotF64)?;

        visitor.visit_f64(f64)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &self.raw {
            Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
            Cow::Owned(s) => visitor.visit_str(s),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match &mut self.raw {
            Cow::Borrowed(s) => visitor.visit_string(s.to_owned()),
            Cow::Owned(s) => visitor.visit_string(take(s)),
        }
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.raw.is_empty() {
            return visitor.visit_none();
        }

        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.raw.is_empty() {
            return visitor.visit_unit();
        }

        Err(Error::ValueNotUnit)
    }

    fn deserialize_unit_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_map<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

pub struct Deserializer<'a> {
    input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'de> de::Deserializer<'de> for &mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_bool<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_i64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u8<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u16<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_u64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_f32<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_f64<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_char<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_str<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_string<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_bytes<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_option<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_unit<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_unit_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_newtype_struct<V>(self, _: &'static str, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_seq<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_tuple<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _: &'static str,
        _: usize,
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_map(VarMapAccess::new(self.input))
    }

    fn deserialize_struct<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }

    fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(Error::UnsupportedDeserialization)
    }
}

struct VarMapAccess<'a> {
    lines: crate::std::str::Lines<'a>,
    current_value: Option<Cow<'a, str>>,
}

impl<'a> VarMapAccess<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            lines: input.lines(),
            current_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for VarMapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        for line in self.lines.by_ref() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let (key, val) = parse_str(trimmed)?;

            self.current_value = Some(val);

            return seed.deserialize(StrDeserializer::new(key)).map(Some);
        }
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = self
            .current_value
            .take()
            .ok_or_else(|| Error::Custom("value called before key".into()))?;

        seed.deserialize(&mut ValueDeserializer { raw: val })
    }
}

pub fn from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T, Error> {
    T::deserialize(&mut Deserializer::new(input))
}

#[cfg(test)]
mod tests {
    use super::{Error, parse_str};

    #[test]
    fn invalid_str() {
        assert_eq!(parse_str(""), Err(Error::EmptyString));
        assert_eq!(
            parse_str("just some string"),
            Err(Error::MissingEqualSeparator)
        );
        assert_eq!(parse_str("=value"), Err(Error::EmptyIdentifier));
        assert_eq!(parse_str("1=value"), Err(Error::IdentifierStartsWithDigit));
        assert_eq!(
            parse_str("❌=❓"),
            Err(Error::InvalidIdentifier {
                char: '❌',
                index: 0
            })
        );

        assert_eq!(
            parse_str("value=\'Hello World"),
            Err(Error::ValueUnterminatedSingleQuote { index: 6 })
        );
        assert_eq!(
            parse_str("Identifier=Hello World\'"),
            Err(Error::ValueUnterminatedSingleQuote { index: 22 })
        );
        assert_eq!(
            parse_str("Identifier=\'Hello\' \'World"),
            Err(Error::ValueUnterminatedSingleQuote { index: 19 })
        );

        assert_eq!(
            parse_str("Identifier=\"Hello World"),
            Err(Error::ValueUnterminatedDoubleQuote { index: 11 })
        );
        assert_eq!(
            parse_str("Identifier=Hello World\""),
            Err(Error::ValueUnterminatedDoubleQuote { index: 22 })
        );
        assert_eq!(
            parse_str("Identifier=\"Hello\" \"World"),
            Err(Error::ValueUnterminatedDoubleQuote { index: 19 })
        );

        assert_eq!(
            parse_str("Identifier=Hello\\"),
            Err(Error::ValueDanglingEscape)
        );
    }
}
