//! Protocol definitions for AIRMASH. This crate provides a strongly typed
//! interface for communicating with an airmash server. It also provides an
//! implementation of the airmash v5 protocol along with serde definitions for
//! all interface structs. This is enough to communicate with any existing
//! airmash server over a websocket connection.
//!
//! # Library Usage
//! The library is designed to be straightforward to use. First construct the
//! appropriate packet from either [server packets](crate::ServerPacket) or
//! [client packets](crate::ClientPacket). From there you can serialize it to
//! bytes via [`v5::serialize`](crate::v5::serialize).
//!
//! ```
//! # use airmash_protocol::{ClientPacket, KeyCode, v5, client::Key};
//! # fn main() -> Result<(), v5::Error> {
//! let packet = ClientPacket::from(Key {
//!   seq: 0,
//!   key: KeyCode::Fire,
//!   state: true
//! });
//!
//! let bytes = airmash_protocol::v5::serialize(&packet)?;
//! // ... send bytes to server
//! # Ok(())
//! # }
//! ```
//!
//! Note that while the individual packet types are serializable individually
//! the only types expected to be passed between clients and servers are
//! [`ServerPacket`] and [`ClientPacket`].
//!
//! For deserialization you can use [`v5::deserialize`] on any byte slice.
//! ```
//! # use airmash_protocol::{ClientPacket, v5};
//! # fn get_some_bytes() -> Vec<u8> { return vec![5]; }
//! # fn main() -> Result<(), v5::Error> {
//! let bytes: Vec<u8> = get_some_bytes();
//! let packet: ClientPacket = airmash_protocol::v5::deserialize(&bytes)?;
//! // ... do stuff with packet
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", macro_use)]
extern crate serde1 as serde;

#[macro_use]
mod detail;

mod enums;
mod error;
mod packets;
mod types;

mod client_packet;
mod server_packet;

#[cfg(feature = "serde")]
pub mod custom;

pub mod v5;

pub use self::client_packet::ClientPacket;
pub use self::enums::*;
pub use self::packets::*;
pub use self::server_packet::ServerPacket;
pub use self::types::*;
pub use crate::error::EnumValueOutOfRangeError;
