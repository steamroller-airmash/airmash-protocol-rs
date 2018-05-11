
mod client_visitor {
    use client::*;
    use de::from_bytes;

    /// A visitor for decoding messages.
    /// 
    /// This permits for messages to be decoded without
    /// copying them into an unnecessarily large enum
    /// just to unwrap it again. Passing decoded messages
    /// through multiple visitors is left to the client.
    /// 
    /// This visitor will probably not be useful for 
    /// those implementing a client (e.g. bot or other)
    /// for airmash. This would mainly be useful for 
    /// those implementing a server.
    pub trait ClientVisitor {
        type Result;

        fn visit_login(&self, p: &Login) -> Result;
        fn visit_backup(&self, p: &Backup) -> Result;
        fn visit_horizon(&self, p: &Horizon) -> Result;
        fn visit_ack(&self) -> Result;
        fn visit_key(&self, p: &Key) -> Result;
        fn visit_pong(&self, p: &Pong) -> Result;
        fn visit_command(&self, p: &Command) -> Result;
        fn visit_score_detailed(&self) -> Result;
        fn visit_chat(&self, p: &Chat) -> Result;
        fn visit_whisper(&self, p: &Whisper) -> Result;
        fn visit_say(&self, p: &Say) -> Result;
        fn visit_votemute(&self, p: &VoteMute) -> Result;
        fn visit_localping(&self, p: &LocalPing) -> Result;
    }

    enum Codes {
        Login = 0,
        Backup = 1,
        Horizon = 2,
        Ack = 5,
        Pong = 6,
        Key = 10,
        Command = 11,
        ScoreDetailed = 12,
        Chat = 20,
        Whisper = 21,
        Say = 22,
        TeamChat = 23,
        VoteMute = 24,
        LocalPing = 255
    };

    pub fn visit<V>(
        bytes: &[u8], 
        visitor: &V
    ) -> Result<V::Result, Serializer::Error> 
    where
        V: ClientVisitor
    {
        let rest = &bytes[1..];

        Ok(match bytes[0] {
            Codes::Login => visitor.visit_login(&from_bytes(rest)?),
            Codes::Backup => visitor.visit_backup(&from_bytes(rest)?),
            Codes::Horizon => visitor.visit_horizon(&from_bytes(rest)?),
            Codes::Ack => visitor.visit_ack(),
            Codes::Pong => visitor.visit_pong(&from_bytes(rest)?),
            Codes::Key => visitor.visit_command(&from_bytes(rest)?),
            Codes::Command => visitor.visit_key(&from_bytes(rest)?),
            Codes::ScoreDetailed => visitor.visit_score_detailed(),
            Codes::Chat => visitor.visit_chat(&from_bytes(rest)?),
            Codes::Whisper => visitor.visit_whisper(&from_bytes(rest)?),
            Codes::Say => visitor.visit_say(&from_bytes(rest)?),
            Codes::VoteMute => visitor.visit_votemute(&from_bytes(rest)?),
            Codes::LocalPing => visitor.visit_localping(&from_bytes(rest)?),
            // TODO: Fold this into result type
            _ => panic!("Unknown command type")
        })
    }
}

mod server_visitor {
    use server::*;
    use de::from_bytes;

    
}
