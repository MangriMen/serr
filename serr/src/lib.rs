mod serialized_error;

pub use serialized_error::*;

#[cfg(feature = "derive")]
pub use serr_derive::SerializeError;
