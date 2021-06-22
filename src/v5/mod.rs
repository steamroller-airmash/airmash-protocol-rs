//! Serialization and deserialization for the airmash v5 protocol.

mod error;
mod protocol;

pub use self::error::{Error, ErrorExt, ErrorKind};
pub use self::protocol::{AirmashDeserializerV5, AirmashSerializerV5, DeserializeV5, SerializeV5};
