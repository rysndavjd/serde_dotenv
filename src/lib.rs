mod error;

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

extern crate alloc;

use crate::error::Error;
use crate::std::fmt;
use serde::{
    Deserialize,
    de::{
        Deserializer, Error as DeError, IgnoredAny, IntoDeserializer, MapAccess, SeqAccess,
        Unexpected, Visitor,
    },
};

struct Var {
    identifier: String,
    value: String,
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

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let identifier = seq
            .next_element()?
            .ok_or_else(|| DeError::invalid_length(0, &self))?;
        let value = seq
            .next_element()?
            .ok_or_else(|| DeError::invalid_length(0, &self))?;

        Ok(Var { identifier, value })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let (mut identifier, mut value) = (None, None);

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "identifier" => identifier = Some(map.next_value()?),
                "value" => value = Some(map.next_value()?),
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        Ok(Var {
            identifier: identifier.ok_or_else(|| DeError::missing_field("identifier"))?,
            value: value.ok_or_else(|| DeError::missing_field("value"))?,
        })
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: DeError,
    {
        let (i, v) = v
            .split_once('=')
            .ok_or(E::custom(format!("missing '=' in \"{}\"", v)))?;

        if i.is_empty() {
            return Err(E::invalid_length(0, &self));
        }

        if i.chars()
            .nth(0)
            .expect("should not be empty")
            .is_ascii_digit()
            || !i.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(E::invalid_value(Unexpected::Str(i), &self));
        }

        if !v
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '\'' || c == '\"')
        {
            return Err(E::invalid_value(Unexpected::Str(v), &self));
        }

        todo!()
    }
}

// impl<'de> IntoDeserializer<'de, Error> for Var {
//     type Deserializer = Self;

//     fn into_deserializer(self) -> Self::Deserializer {
//         self
//     }
// }

// impl<'de> Deserializer<'de> for Var {
//     type Error = Error;

//     fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         self.value.into_deserializer().deserialize_any(visitor)
//     }

//     fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         match self.value.parse::<bool>() {
//             Ok(t) => t.into_deserializer().deserialize_bool(visitor),
//             Err(e) => {
//                 todo!()
//             }
//         }
//     }

//     fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         match self.value.parse::<i8>() {
//             Ok(t) => t.into_deserializer().deserialize_i8(visitor),
//             Err(e) => {
//                 todo!()
//             }
//         }
//     }

//     fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         match self.value.parse::<i16>() {
//             Ok(t) => t.into_deserializer().deserialize_i16(visitor),
//             Err(e) => {
//                 todo!()
//             }
//         }
//     }

//     fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         match self.value.parse::<i32>() {
//             Ok(t) => t.into_deserializer().deserialize_i32(visitor),
//             Err(e) => {
//                 todo!()
//             }
//         }
//     }

//     fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         match self.value.parse::<i64>() {
//             Ok(t) => t.into_deserializer().deserialize_i64(visitor),
//             Err(e) => {
//                 todo!()
//             }
//         }
//     }

//     fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_newtype_struct<V>(
//         self,
//         name: &'static str,
//         visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     fn deserialize_enum<V>(
//         self,
//         name: &'static str,
//         variants: &'static [&'static str],
//         visitor: V,
//     ) -> Result<V::Value, Self::Error>
//     where
//         V: serde::de::Visitor<'de>,
//     {
//         todo!()
//     }

//     serde::forward_to_deserialize_any! {
//         char str string unit
//         bytes byte_buf map unit_struct tuple_struct
//         identifier tuple ignored_any
//         struct
//     }
// }
