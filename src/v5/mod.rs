//! Serialization and deserialization for the airmash v5 protocol.

#[macro_use]
mod macros;

mod client;
mod error;
mod protocol;
mod types;
mod server;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub use self::error::{Error, ErrorExt, ErrorKind};
pub use self::protocol::{AirmashDeserializerV5, AirmashSerializerV5, DeserializeV5, SerializeV5};

pub fn serialize<T: SerializeV5>(value: &T) -> Result<Vec<u8>> {
  let mut data = vec![];
  let mut ser = AirmashSerializerV5::new(&mut data);
  ser.serialize(value)?;
  Ok(data)
}

pub fn deserialize<'de, T: DeserializeV5<'de>>(data: &'de [u8]) -> Result<T> {
  let mut de = AirmashDeserializerV5::new(data);
  let val = de.deserialize()?;

  if !de.remainder().is_empty() {
    return Err(Error::new(ErrorKind::UnexpectedDataRemaining));
  }

  Ok(val)
}
