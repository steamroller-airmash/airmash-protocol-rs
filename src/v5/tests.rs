use super::serialize;
use crate::{
  server::PlayerUpdate,
  v5::{AirmashDeserializerV5, AirmashSerializerV5},
  ServerKeyState, ServerPacket, Upgrades, Vector2,
};

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
