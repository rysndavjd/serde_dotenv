mod error;

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

extern crate alloc;

mod de;
mod ser;

pub use crate::{
    de::{VarsDeserializer, from_str},
    error::Error,
};
