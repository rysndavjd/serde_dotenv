#![no_std]

extern crate alloc;
#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;
#[cfg(any(feature = "std", test))]
extern crate std;

mod de;
mod error;
mod ser;

pub use crate::{
    de::{Deserializer, from_str},
    error::Error,
    ser::{
        Serializer, to_string_compact_posix, to_string_pretty_posix, to_vec_compact_posix,
        to_vec_pretty_posix, to_writer_compact_posix, to_writer_pretty_posix,
    },
};

// #[cfg(feature = "std")]
// compile_error!("");
