//! Packets that the client sends to the server.

mod v5;

use crate::enums::KeyCode;
use bstr::BString;

/// Opening packet for opening a second server connection for the same client.
///
/// This packet is used to allow for multiple websocket connections to the
/// airmash server. To open a second connection, open a websocket connection to
/// the server, then send this packet as the first packet instead of sending
/// [`Login`](struct.login.html). The server will respond to client packets sent
/// through this channel, allowing for some reduction in head of line blocking.
#[derive(Clone, Debug)]
pub struct Backup {
  pub token: BString,
}

/// Say something in public chat.
#[derive(Clone, Debug)]
pub struct Chat {
  pub text: BString,
}

/// A free form command to be sent to the server. This is used for changing
/// flags, respawning, spectating players, and selecting upgrades.
///
/// # Changing a flag
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "flag".to_string(),
///     // Set to desired flag code,
///     // unknown will result in UN flag.
///     // Here we will set to the UN flag.
///     data: "XX".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
/// ```
///
/// # Respawning as a plane
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "respawn".to_string(),
///     // Choose the plane type here,
///     // each type is associated with
///     // an integer. Here we will pick
///     // predator.
///     data: "1".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
/// ```
///
/// # Selecting Upgrades
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "upgrade".to_string(),
///     // Choose upgrade type here.
///     // Here speed should be 1.
///     data: "1".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
/// ```
///
/// # Going into spectate or spectating a different player
/// ```
/// # extern crate airmash_protocol;
/// # use airmash_protocol::client::Command;
/// # fn main() {
/// let cmd = Command {
///     com: "spectate".to_string(),
///     // This can either be a player id, or
///     // one of -1, -2, or -3. -3 will force
///     // the player to go into spectate,
///     // -1 switches focus to the next player,
///     // and -2 switches focus to the previous
///     // player. Here we will force the player
///     // to go into spectate.
///     data: "-3".to_string()
/// };
///
/// // Serialize and send to server here...
/// # }
#[derive(Clone, Debug)]
pub struct Command {
  /// The command to send to the server. The
  /// official server recognizes the commands
  /// `"spectate"`, `"upgrade"`, `"flag"`, and
  /// `"respawn"`.
  pub com: BString,
  /// The data associated with the command,
  /// value values epend on the given command.
  pub data: BString,
}

/// Sent periodically by the client to indicate that it is still alive to the
/// server.
#[derive(Copy, Clone, Debug, Default)]
pub struct Ack;

/// Request a detailed score packet from the server.
#[derive(Copy, Clone, Debug, Default)]
pub struct ScoreDetailed;

/// Packet to tell the server to resize the horizon for the client.
///
/// In theory this should expand the visible range for the client, in practice
/// the official server appears to ignore these packets.
#[derive(Copy, Clone, Debug)]
pub struct Horizon {
  pub horizon_x: u16,
  pub horizon_y: u16,
}

/// Purpose unknown, doesn't appear to be used in the official client.
#[derive(Copy, Clone, Debug)]
pub struct LocalPing {
  pub auth: u32,
}

/// Initial packet sent to log in to the server.
///
/// This sent to the server when the player first joins.
#[derive(Clone, Debug)]
pub struct Login {
  /// The current protocol version. Should always be 5 as of the writing of this
  /// documentation.
  pub protocol: u8,
  /// The name that the player wishes to be called on the server. The actual
  /// name of the player given by the server will be in the
  /// [`Login`](../server/struct.login.html) packet returned by the server.
  pub name: BString,
  /// A session token for the current player. This session token is the way that
  /// a player would log in to the server. If the player does not wish to be
  /// logged on to the server then a session token of `"none"` will suffice.
  pub session: BString,
  /// Should set the size of the horizon beyond which game updates (missile
  /// updates and player updates) are not sent to the client. In practice, this
  /// doesn't appear to be used by the official server.
  pub horizon_x: u16,
  /// Same as `horizon_x` but in the y direction.
  pub horizon_y: u16,
  /// The desired flag of the player. This should be the ISO-2 country code
  /// corresponding to the flag that the player wishes to take. It may also be
  /// one of the special flag codes for non-country flags.
  ///
  /// If the flag code passed in is not one of the ones for which there is a
  /// known (to the server) flag, then the player will be assigned to UN flag
  /// (in the official server).
  pub flag: BString,
}

/// Send a key update for the client.
///
/// Notes:
/// - `seq` should be monotonically increasing with every key press.
/// - `state`: `true` -> pressed, `false` -> released.
#[derive(Copy, Clone, Debug)]
pub struct Key {
  pub seq: u32,
  pub key: KeyCode,
  pub state: bool,
}

/// Response packet to server [`Ping`](../server/struct.ping.html)s.
#[derive(Copy, Clone, Debug)]
pub struct Pong {
  /// The ping number, should correspond to the `num` field within in the
  /// [`Ping`](../server/ping.html) packet sent by the server.
  pub num: u32,
}

/// Say a message in a chat bubble
#[derive(Clone, Debug)]
pub struct Say {
  pub text: BString,
}

/// Send a message to your team.
#[derive(Clone, Debug)]
pub struct TeamChat {
  pub text: BString,
}

/// Vote to mute a player
#[derive(Copy, Clone, Debug)]
pub struct VoteMute {
  pub id: u16,
}

/// Send a whisper to another player.
#[derive(Clone, Debug)]
pub struct Whisper {
  pub id: u16,
  pub text: BString,
}
