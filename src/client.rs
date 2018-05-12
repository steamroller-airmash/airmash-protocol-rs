//! Messages sent from client to server

serde_decls! {
    /* READ BEFORE EDITING THIS FILE!
        Serialization/Deserialization is done in
        the order that the fields are declared.
        Changing the order of the fields without
        being aware of this will break things!
    */


    /// Login packet
    #[derive(Default, Clone, Debug)]
    pub struct Login {
        pub protocol: u8,
        pub name: text,
        pub session: text,
        pub horizon_x: u16,
        pub horizon_y: u16,
        pub flag: text
    }

    #[derive(Default, Clone, Debug)]
    pub struct Backup {
        pub token: text
    }

    #[derive(Default, Clone, Debug, Copy)]
    pub struct Horizon {
        pub horizon_x: u16,
        pub horizon_y: u16
    }

    // Could include this, no point though
    //pub struct Ack { }

    #[derive(Default, Clone, Debug, Copy)]
    pub struct Pong {
        pub num: u32
    }

    #[derive(Default, Clone, Debug, Copy)]
    pub struct Key {
        pub seq: u32,
        pub key: u8,
        pub state: bool
    }

    #[derive(Default, Clone, Debug)]
    pub struct Command {
        pub com: text,
        pub data: text
    }

    //pub struct ScoreDetailed { }

    #[derive(Default, Clone, Debug)]
    pub struct Chat {
        pub text: text
    }

    #[derive(Default, Clone, Debug)]
    pub struct Whisper {
        pub id: u16,
        pub text: text
    }

    #[derive(Default, Clone, Debug)]
    pub struct Say {
        pub text: text
    }

    #[derive(Default, Clone, Debug)]
    pub struct TeamChat {
        pub text: text
    }

    #[derive(Default, Clone, Debug, Copy)]
    pub struct VoteMute {
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


