
//! **THIS LIBRARY IS IN ALPHA!**
//! **USE AT YOUR OWN RISK**
//! 
//! This library contains definitions 
//! and serialization/deserialization methods
//! for the protocol for the game 
//! [`AIRMASH`](https://airma.sh). It implements
//! serialization and deserialization and is 
//! intended to be used for creating bots or 
//! servers using this protocol.
//! 
//! # Basic Usage
//! Serialization and deserialization of packets
//! is simple on the caller's side. All that
//! must be done is to call 
//! [`from_bytes`](fn.from_bytes.html) to
//! deserialize and to call
//! [`to_bytes`](fn.to_bytes.html) to serialize.
//! The [`ServerPacket`](server/struct.serverpacket.html)
//! and [`ClientPacket`](client/struct.clientpacket.html)
//! structs will take care of decoding the message 
//! type and message fields for you as well as 
//! encoding them. If an invalid message is passed 
//! then deserialize will return an error indicating
//! what went wrong. (Note: it will currently panic
//! in some cases. See TODOs). If an error occurse while
//! serializing, an error will be returned too. 
//! (Note: will panic currently. See TODOs).
//! 
//! # Client-Side
//! Clients using this library will be deserializing
//! packets into the 
//! [`ServerPacket`](server/enum.serverpacket.html)
//! enum and will be responding by serializing
//! [`ClientPacket`](client/enum.clientpacket.html)s
//! and sending those to the server.
//! 
//! # Server-Side
//! Servers using this library will be doing the 
//! opposite of clients. They will deserialize
//! [`ServerPacket`](server/enum.serverpacket.html)s
//! and will be serializing
//! [`ClientPacket`](client/enum.clientpacket.html)s.
//! 
//! # TODOs
//! There is still a bunch of things that can be 
//! improved within the library:
//! 
//! - Change plane types to an enum. (Currently a `u8`)
//! - Change teams to an enum. (Currently a `u8`)
//! - Remove all `panic!` invocations from serializatation
//!   and deserialization code and replace them with 
//!   error codes.
//! - Document [`Error`](enum.error.html).
//! - Complete packet field documentation/figure out
//!   what all packet fields actually do.
//! - Complete documentation of BTR 
//!   [`ServerCustom`](server/struct.servercustom.html)
//!   data format.
//! - Add feature-gated serde derivations for all packet
//!   types and enums.
//! - Write unit tests for all serialization and deserialization
//!   groups within [`field.rs`](../src/airmash_protocol/field.rs.html).
//! - More internal documentation on specific protocol data types.
//!   This should probably go within 
//!   [`field.rs`](../src/airmash_protocol/field.rs.html) too.
//! - Change `key` and `state` within
//!   [`Key`](client/struct.key.html) to be enums of their respective
//!   values.
//! 

#[macro_use]
mod macros;

mod serde;

mod de;
mod error;
mod field;
mod ser;

mod impls;
mod packet_impls;
mod codes;

mod datatypes;

mod field_tests;

pub mod client;
pub mod server;

pub use ser::to_bytes;
pub use de::from_bytes;
pub use error::Error;

pub use client::ClientPacket;
pub use server::ServerPacket;

pub use datatypes::*;
