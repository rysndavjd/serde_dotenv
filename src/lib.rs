#![no_std]

extern crate alloc;
#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;
#[cfg(any(feature = "std", test))]
extern crate std;

mod common;
mod de;
mod error;
mod ser;

pub use crate::{
    de::{Deserializer, from_str},
    error::Error,
    ser::Serializer,
};

#[cfg(feature = "writer")]
pub use crate::ser::{to_string, to_vec, to_writer};

// #[cfg(feature = "std")]
// compile_error!("");
