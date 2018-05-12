//! Messages sent from client to server

serde_decls! {
    /* READ BEFORE EDITING THIS FILE!
        Serialization/Deserialization is done in
        the order that the fields are declared.
        Changing the order of the fields without
        being aware of this will break things!
    */


    /// Initial packet sent to log in to
    /// the server.
    /// 
    /// This is sent to the server 
    /// when the player first joins.
    #[derive(Default, Clone, Debug)]
    pub struct Login {
        /// The current protocol version.
        /// Should always be 5 as of the 
        /// writing of this documentation.
        pub protocol: u8,
        /// The name that the player wishes
        /// to be called on the server. The 
        /// actual name of the player given 
        /// by the server will be returned 
        /// in the [`Login`](../server/struct.login.html)
        /// packet returned by the server.
        pub name: text,
        /// A session token for the current
        /// player. This is how a player logs
        /// into the server. If the player
        /// logging in wishes to be associated 
        /// with an account, this must be
        /// set. Otherwise, `"none"` works
        /// to avoid being given an account.
        pub session: text,
        /// Theoretically should set the size
        /// of the horizon beyond which players
        /// are not sent to the server. In practice
        /// doesn't appear to do anything.
        pub horizon_x: u16,
        /// Theoretically should set the size
        /// of the horizon beyond which players
        /// are not sent to the server. In practice
        /// doesn't appear to do anything.
        pub horizon_y: u16,
        /// The flag of the player, it should be a 
        /// 2-letter ISO country code corresponding
        /// to the country with the desired flag.
        pub flag: text
    }

    #[derive(Default, Clone, Debug)]
    pub struct Backup {
        pub token: text
    }

    /// In theory this should resize the horizon
    /// of the player. In practice the airmash
    /// server appears to ignore these packets.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct Horizon {
        pub horizon_x: u16,
        pub horizon_y: u16
    }

    // Could include this, no point though
    //pub struct Ack { }

    /// Response packet to the server
    /// [`Ping`](../server/struct.ping.html)
    /// packet.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct Pong {
        /// The ping number, should correspond 
        /// to the `num` field within in the 
        /// [`Ping`](../server/ping.html) packet
        /// sent by the server.
        pub num: u32
    }

    /// Send keystate of client
    #[derive(Default, Clone, Debug, Copy)]
    pub struct Key {
        pub seq: u32,
        /// Keycodes as follows:
        /// 
        /// - `UP: 1`
        /// - `DOWN: 2`
        /// - `LEFT: 3`
        /// - `RIGHT: 4`
        /// - `FIRE: 5`
        /// - `SPECIAL: 6`
        pub key: u8,
        /// True for pressed, false for released
        /// 
        /// TODO: Verify this
        pub state: bool
    }

    /// A free form command to be sent to the server.
    /// This is used for changing flags, respawning,
    /// spectating players, and selecting upgrades.
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
    /// 
    #[derive(Default, Clone, Debug)]
    pub struct Command {
        /// The command to send to the server,
        /// this can be one of `"spectate"`,
        /// `"upgrade"`, `"flag"`, or 
        /// `"respawn"`.
        pub com: text,
        /// The data associated with the command,
        /// valid values depend on the given command.
        pub data: text
    }

    //pub struct ScoreDetailed { }

    /// Say something in chat.
    #[derive(Default, Clone, Debug)]
    pub struct Chat {
        /// Text of the chat message.
        pub text: text
    }

    /// Send a whisper to a given player.
    #[derive(Default, Clone, Debug)]
    pub struct Whisper {
        /// The id of the player to send 
        /// the whisper to.
        pub id: u16,
        /// Contents of the whisper.
        pub text: text
    }

    /// Say a message in a chat bubble.
    #[derive(Default, Clone, Debug)]
    pub struct Say {
        /// The text within the chat bubble.
        pub text: text
    }

    /// Send a message to your team.
    #[derive(Default, Clone, Debug)]
    pub struct TeamChat {
        /// The message text.
        pub text: text
    }

    /// Issue a vote to mute a player.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct VoteMute {
        /// The id of the player to mute.
        pub id: u16
    }

    #[derive(Default, Clone, Debug, Copy)]
    pub struct LocalPing {
        pub auth: u32
    }
}

/// All possible client packets.
/// 
/// This contains all valid packets that
/// the client can send to the server
/// (in the current version of the airmash
/// protocol). It can be serialized and 
/// deserialized to/from byte buffers
/// using [`to_bytes`](fn.to_bytes.html)
/// and [`from_bytes`](fn.from_bytes.html).
/// 
/// Some packets don't contain any data, these
/// packets do not have an associated struct
/// and as such are just empty variants within
/// this enum.
#[derive(Clone, Debug)]
pub enum ClientPacket {
    Login(Login),
    Backup(Backup),
    Horizon(Horizon),
    Ack,
    Pong(Pong),
    Key(Key),
    Command(Command),
    ScoreDetailed,
    Chat(Chat),
    TeamChat(TeamChat),
    Whisper(Whisper),
    Say(Say),
    VoteMute(VoteMute),
    LocalPing(LocalPing),
}


