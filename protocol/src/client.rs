//! Messages sent from client to server

serde_decls! {
    /* READ BEFORE EDITING THIS FILE:
        Serialization/Deserialization is done in
        the order that the fields are declared.
        Changing the order of the fields without
        being aware of this will break things!
    */

    
    /// Login packet
    pub struct Login {
        pub protocol: u8,
        pub name: text,
        pub session: text,
        pub horizon_x: u16,
        pub horizon_y: u16,
        pub flag: text
    }

    pub struct Backup {
        pub token: text
    }

    pub struct Horizon {
        pub horizon_x: u16,
        pub horizon_y: u16
    }

    pub struct Ack { }

    pub struct Pong {
        pub num: u32
    }

    pub struct Key {
        pub seq: u32,
        pub key: u8,
        pub state: bool
    }

    pub struct Command {
        pub com: text,
        pub data: text
    }

    pub struct ScoreDetailed { }

    pub struct Chat {
        pub text: text
    }

    pub struct Whisper {
        pub id: u16,
        pub text: text
    }

    pub struct Say {
        pub text: text
    }

    pub struct VoteMute {
        pub id: u16
    }

    pub struct LocalPing {
        pub auth: u32
    }
}
