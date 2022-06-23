use approx::*;

use super::serialize;
use crate::server::PlayerUpdate;
use crate::types::VectorExt;
use crate::v5::{AirmashDeserializerV5, AirmashSerializerV5};
use crate::{GameType, ServerKeyState, ServerPacket, Upgrades, Vector2};

#[test]
fn reference_deserialize_player_update() {
  let reference = vec![
    12, 10, 10, 0, 0, 11, 11, 0, 24, 190, 139, 0, 48, 126, 0, 204, 12, 102, 134, 62, 106,
  ];

  let packet = ServerPacket::PlayerUpdate(PlayerUpdate {
    clock: 0x0A0A,
    id: 0x0B0B,
    keystate: ServerKeyState {
      up: false,
      down: false,
      left: false,
      right: false,
      boost: false,
      strafe: false,
      stealth: false,
      flagspeed: false,
    },
    upgrades: Upgrades {
      speed: 0,
      shield: true,
      inferno: true,
    },
    pos: Vector2::new(1503.0, -232.0),
    rot: 0.49987793,
    speed: Vector2::new(0.99975586, -3.3996582),
  });

  let actual = serialize(&packet).unwrap();

  assert_eq!(reference, actual);
}

#[test]
fn reference_deserialize_game_flag() {
  let reference = vec![30, 1, 8, 4, 0, 192, 92, 0, 188, 129, 0, 3, 2];
  let packet: ServerPacket = crate::v5::deserialize(&reference)
    .unwrap_or_else(|e| panic!("Failed to deserialize reference packet: {}", e));

  let inner = match packet {
    ServerPacket::GameFlag(p) => p,
    _ => panic!("Wrong packet type"),
  };

  assert_abs_diff_eq!(inner.pos.x, -4512.0, epsilon = 0.01);
  assert_abs_diff_eq!(inner.pos.y, 222.0, epsilon = 0.01);
}

#[test]
fn reference_serialize_game_flag() {
  use crate::server::GameFlag;
  use crate::FlagUpdateType;

  let packet = ServerPacket::GameFlag(GameFlag {
    ty: FlagUpdateType::Position,
    flag: 8,
    id: Some(4),
    pos: Vector2::new(-9670.0, -1470.0),
    blueteam: 3,
    redteam: 2,
  });
  let bytes = crate::v5::serialize(&packet).unwrap_or_else(|e| panic!("{}", e));

  assert_eq!(bytes, &[30, 1, 8, 4, 0, 116, 52, 0, 132, 116, 0, 3, 2]);
}

#[test]
fn uint24_serde() {
  fn serialize(v: u32) -> Vec<u8> {
    let mut out = Vec::new();
    let mut ser = AirmashSerializerV5::new(&mut out);
    ser.serialize_u24(v).unwrap();
    out
  }

  fn deserialize(data: [u8; 3]) -> u32 {
    let mut de = AirmashDeserializerV5::new(&data[..]);
    de.deserialize_u24().unwrap()
  }

  assert_eq!(&serialize(0x0000AA), &[0x00, 0x00, 0xAA]);
  assert_eq!(&serialize(0x030201), &[2, 3, 1]);

  assert_eq!(deserialize([2, 3, 1]), 0x030201);
}

#[test]
fn login2_extra_invalid() {
  #[rustfmt::skip]
  let bytes: Vec<u8> = vec![
    /* packet  */ crate::server::Login::V5_PACKET_NO,
    /* success */ 1,
    /* id      */ 1, 0,
    /* team    */ 1, 0,
    /* clock   */ 0, 0, 0, 0,
    /* token   */ 0,          // empty string
    /* ty      */ 1,          // FFA
    /* room    */ 0,          // empty string
    /* players */ 0, 0,       // empty array
    /* config  */ 0, 0,       // empty string
    /* bots    */ 32, 0,      // bad array
  ];

  let result = crate::v5::deserialize::<ServerPacket>(&bytes)
    .expect_err("deserialization was supposed to fail");

  eprintln!("{}", result);

  assert_eq!(result.context(), ["id", "bots", "Login2", "ServerPacket"]);
}

#[test]
fn login2_valid() {
  use crate::packets::server::{Login, Login2};

  #[rustfmt::skip]
  let bytes: Vec<u8> = vec![
    /* packet  */ crate::server::Login::V5_PACKET_NO,
    /* success */ 1,
    /* id      */ 1, 0,
    /* team    */ 1, 0,
    /* clock   */ 0, 0, 0, 0,
    /* token   */ 0,          // empty string
    /* ty      */ 1,          // FFA
    /* room    */ 4, b't', b'e', b's', b't',
    /* players */ 0, 0,       // empty array
    /* config  */ 4, 0, b't', b'e', b's', b't',
    /* bots    */ 1, 0,
      /* id    */ 1, 1     
  ];

  let packet =
    crate::v5::deserialize::<ServerPacket>(&bytes).expect("deserialization was supposed to fail");

  assert!(matches!(
    packet,
    ServerPacket::Login2(Login2 {
      login: Login {
        success: true,
        id: 1,
        team: 1,
        clock: 0,
        ty: GameType::FFA,
        ..
      },
      ..
    })
  ));

  let packet = match packet {
    ServerPacket::Login2(login2) => login2,
    _ => unreachable!(),
  };

  assert_eq!(packet.config, "test");
  assert_eq!(packet.room, "test");
  assert_eq!(packet.bots.len(), 1);
  assert_eq!(packet.bots[0].id, 0x101);
}
