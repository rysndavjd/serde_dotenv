use crate::{common::State, error::Error, std::num::FpCategory};
use alloc::{borrow::Cow, string::String, vec::Vec};
use lexical_core::FormattedSize;
use serde::ser::{self, Impossible, Serialize};

#[cfg(all(feature = "std", feature = "writer"))]
use std::io::Write;

#[cfg(all(feature = "no_std", feature = "writer"))]
use embedded_io::Write;

pub fn validate_value<'a>(val: &'a str) -> Result<Cow<'a, str>, Error> {
    if !val.contains(['"', '\'', '\\', '|', '&', ';', '<', '>', '(', ')', '`']) {
        return Ok(Cow::Borrowed(val));
    }

    let mut output = String::new();

    let mut state = State::Unquoted;
    let mut quote_pos = 0usize;

    let mut chars = val.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c == '\\' && state != State::SingleQuoted {
            match chars.next() {
                Some((_, escaped_char)) => {
                    output.push(c);
                    output.push(escaped_char);
                    continue;
                }
                None => return Err(Error::ValueDanglingEscape),
            }
        }

        match state {
            State::Unquoted => match c {
                '\'' => {
                    quote_pos = i;
                    state = State::SingleQuoted;
                }
                '"' => {
                    quote_pos = i;
                    state = State::DoubleQuoted;
                }
                '|' | '&' | ';' | '<' | '>' | '(' | ')' | '`' | '\\' => {
                    return Err(Error::ValueUnescapedShellChar { index: i });
                }
                _ => {}
            },
            State::SingleQuoted if c == '\'' => {
                debug_assert!(i > quote_pos);

                state = State::Unquoted;
                quote_pos = 0;
            }
            State::DoubleQuoted if c == '"' => {
                debug_assert!(i > quote_pos);

                state = State::Unquoted;
                quote_pos = 0;
            }
            _ => (),
        }

        output.push(c);
    }

    match state {
        State::SingleQuoted => {
            return Err(Error::ValueUnterminatedSingleQuote);
        }
        State::DoubleQuoted => {
            return Err(Error::ValueUnterminatedDoubleQuote);
        }
        _ => (),
    }

    Ok(Cow::Owned(output))
}

struct MapKeySerializer<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> ser::Serializer for MapKeySerializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Self::Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_u8(self, _: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(v)
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedSerialization)
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedSerialization)
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
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::UnsupportedSerialization)
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

pub struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    first: bool,
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.first {
            self.ser.writer.write_all(b"\n")?;
        }
        self.first = false;
        key.serialize(MapKeySerializer { ser: self.ser })
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.ser.writer.write_all(b"=")?;
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeMap::end(self)
    }
}

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W>
where
    W: Write,
{
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer { writer }
    }

    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let s = if v {
            b"true" as &[u8]
        } else {
            b"false" as &[u8]
        };
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; i8::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; i16::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; i32::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; i64::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; i128::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u8::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u16::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u32::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u64::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; u128::FORMATTED_SIZE_DECIMAL];
        let s = lexical_core::write(v, &mut buf);
        self.writer.write_all(s)?;

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                return Err(Error::FloatNotFinite);
            }
            _ => {
                let mut buf = [0u8; f32::FORMATTED_SIZE_DECIMAL];
                let s = lexical_core::write(v, &mut buf);
                self.writer.write_all(s)?;
            }
        }

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                return Err(Error::FloatNotFinite);
            }
            _ => {
                let mut buf = [0u8; f64::FORMATTED_SIZE_DECIMAL];
                let s = lexical_core::write(v, &mut buf);
                self.writer.write_all(s)?;
            }
        }

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let value = validate_value(v)?;
        self.writer.write_all(value.as_bytes())?;

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
        self.serialize_str(variant)?;
        self.writer.write_all(b"=")?;
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
        Ok(Compound {
            ser: self,
            first: true,
        })
    }

    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
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

#[cfg(feature = "writer")]
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), Error>
where
    W: Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)
}

#[cfg(feature = "writer")]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::new();
    to_writer(&mut writer, value)?;
    Ok(writer)
}

#[cfg(feature = "writer")]
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec(value)?;
    let string = unsafe { String::from_utf8_unchecked(vec) };
    Ok(string)
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
            name: r#"TEST"#.into(),
            age: 67,
        };

        let o = to_string(&t).unwrap();
    }
}
