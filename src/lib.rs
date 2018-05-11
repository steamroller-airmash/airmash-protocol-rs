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

pub mod client;
pub mod server;

pub use ser::to_bytes;
pub use de::from_bytes;

pub use client::ClientPacket;
pub use server::ServerPacket;
