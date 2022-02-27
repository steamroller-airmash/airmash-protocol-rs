use crate::client::*;
use crate::v5::*;
use crate::ClientPacket;

decl_serde! {
  struct Login {
    protocol,
    name => { serialize_text_small, deserialize_text_small },
    session => { serialize_text_small, deserialize_text_small },
    horizon_x,
    horizon_y,
    flag => { serialize_text_small, deserialize_text_small }
  }

  struct Backup {
    token => { serialize_text_small, deserialize_text_small },
  }

  struct Horizon {
    horizon_x,
    horizon_y,
  }

  struct Pong {
    num,
  }

  struct Key {
    seq,
    key,
    state
  }

  struct Command {
    com => { serialize_text_small, deserialize_text_small },
    data => { serialize_text_small, deserialize_text_small }
  }

  struct Chat {
    text => { serialize_text_small, deserialize_text_small }
  }

  struct Whisper {
    id,
    text => { serialize_text_small, deserialize_text_small },
  }

  struct Say {
    text => { serialize_text_small, deserialize_text_small },
  }

  struct TeamChat {
    text => { serialize_text_small, deserialize_text_small },
  }

  struct VoteMute {
    id
  }

  struct LocalPing {
    auth
  }
}

decl_consts! {
  const Login = 0;
  const Backup = 1;
  const Horizon = 2;
  const Ack = 5;
  const Pong = 6;
  const Key = 10;
  const Command = 11;
  const ScoreDetailed = 12;
  const Chat = 20;
  const Whisper = 21;
  const Say = 22;
  const TeamChat = 23;
  const VoteMute = 25;
  const LocalPing = 225;
}

packet_serde! {
  enum ClientPacket {
    Login(x),
    Backup(x),
    Horizon(x),
    Ack,
    Pong(x),
    Key(x),
    Command(x),
    ScoreDetailed,
    Chat(x),
    TeamChat(x),
    Whisper(x),
    Say(x),
    VoteMute(x),
    LocalPing(x)
  }
}
