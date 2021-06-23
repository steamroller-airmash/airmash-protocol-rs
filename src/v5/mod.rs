//! Serialization and deserialization for the airmash v5 protocol.

#[macro_use]
mod macros;

mod client;
mod error;
mod protocol;
mod server;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub use self::error::{Error, ErrorExt, ErrorKind};
pub use self::protocol::{AirmashDeserializerV5, AirmashSerializerV5, DeserializeV5, SerializeV5};
