use serde::{Serialize, ser};

use crate::error::Error;

pub struct Serializer {
    output: String,
}

// impl<'a> ser::Serializer for &'a mut Serializer {
//     type Ok = ();

//     type Error = Error;

//     type SerializeSeq = Self;
//     type SerializeTuple = Self;
//     type SerializeTupleStruct = Self;
//     type SerializeTupleVariant = Self;
//     type SerializeMap = Self;
//     type SerializeStruct = Self;
//     type SerializeStructVariant = Self;
// }
