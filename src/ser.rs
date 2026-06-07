use crate::{common::QuoteState, error::Error};
use alloc::borrow::Cow;
use itoa::Buffer as IntBuffer;
use serde::ser::{Impossible, Serialize, SerializeStruct, Serializer};
use zmij::Buffer as FloatBuffer;

pub fn validate_value<'a>(val: &'a str) -> Result<Cow<'a, str>, Error> {
    if !val.contains(['"', '\'', '\\', '|', '&', ';', '<', '>', '(', ')', '`']) {
        return Ok(Cow::Borrowed(val));
    }

    let mut output = String::new();

    let mut quote = QuoteState::Unquoted;
    let mut quote_pos = 0usize;

    let mut chars = val.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '\\' && quote != QuoteState::SingleQuoted {
            match chars.next() {
                Some((_, escaped_char)) => {
                    output.push(c);
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
                }
                '"' => {
                    quote_pos = i;
                    quote = QuoteState::DoubleQuoted;
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
            }
            QuoteState::DoubleQuoted if c == '"' => {
                debug_assert!(i > quote_pos);

                quote = QuoteState::Unquoted;
                quote_pos = 0;
            }
            _ => (),
        }

        output.push(c);
    }

    match quote {
        QuoteState::SingleQuoted => {
            return Err(Error::ValueUnterminatedSingleQuote { index: quote_pos });
        }
        QuoteState::DoubleQuoted => {
            return Err(Error::ValueUnterminatedDoubleQuote { index: quote_pos });
        }
        _ => (),
    }

    Ok(Cow::Owned(output))
}

pub struct VarsSerializer {
    output: String,
}

impl VarsSerializer {
    fn new() -> VarsSerializer {
        VarsSerializer {
            output: String::new(),
        }
    }
}

impl<'a> Serializer for &'a mut VarsSerializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = VarsStruct<'a>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.output += if v { "true" } else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buf = IntBuffer::new();

        self.output += buf.format(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut buf = IntBuffer::new();

        self.output += buf.format(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut buf = FloatBuffer::new();

        self.output += buf.format(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.output += validate_value(v)?.as_ref();
        Ok(())
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        variant.serialize(&mut *self)?;
        self.output += "=";
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(VarsStruct {
            entries: Vec::with_capacity(len),
            output: &mut self.output,
        })
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }
}

pub struct VarsStruct<'a> {
    entries: Vec<(&'static str, String)>,
    output: &'a mut String,
}

impl<'a> SerializeStruct for VarsStruct<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut val_serializer = VarsSerializer::new();
        value.serialize(&mut val_serializer)?;

        self.entries.push((key, val_serializer.output));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let lines: Vec<String> = self
            .entries
            .iter()
            .map(|(key, val)| format!("{}={}", key, val))
            .collect();

        self.output.push_str(&lines.join("\n"));

        Ok(())
    }
}

fn to_string<T: ?Sized + Serialize>(value: &T) -> Result<String, Error> {
    let mut serializer = VarsSerializer::new();
    T::serialize(value, &mut serializer)?;
    Ok(serializer.output)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Serialize)]
    struct Test {
        name: String,
        age: u32,
    }

    #[test]
    fn test() {
        let t = Test {
            name: r#"Ry\(\sn"#.into(),
            age: 67,
        };

        let output = to_string(&t).unwrap();

        println!("{}", output);
    }
}
