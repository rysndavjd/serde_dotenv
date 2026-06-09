mod error;

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

extern crate alloc;

mod common;
mod de;
mod ser;

pub use crate::{
    de::{Deserializer, from_str},
    error::Error,
    ser::{Serializer, to_string, to_vec, to_writer},
};
