use crate::types::*;
use crate::v5::*;

impl SerializeV5 for ServerKeyState {
  fn serialize(&self, ser: &mut AirmashSerializerV5) -> Result {
    let val = (self.up as u8)
      | (self.down as u8) << 1
      | (self.left as u8) << 2
      | (self.right as u8) << 3
      | (self.boost as u8) << 4
      | (self.strafe as u8) << 5
      | (self.stealth as u8) << 6
      | (self.flagspeed as u8) << 7;
    val.serialize(ser)
  }
}

impl<'de> DeserializeV5<'de> for ServerKeyState {
  fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
    let val = de.deserialize_u8()?;

    Ok(ServerKeyState {
      up: (val & 0b00000001) != 0,
      down: (val & 0b00000010) != 0,
      left: (val & 0b00000100) != 0,
      right: (val & 0b00001000) != 0,
      boost: (val & 0b00010000) != 0,
      strafe: (val & 0b00100000) != 0,
      stealth: (val & 0b01000000) != 0,
      flagspeed: (val & 0b10000000) != 0,
    })
  }
}

impl SerializeV5 for Upgrades {
  fn serialize(&self, ser: &mut AirmashSerializerV5) -> Result {
    let val: u8 =
      (self.speed & 7) | ((self.shield as u8 & 1) << 3) | ((self.inferno as u8 & 1) << 4);
    val.serialize(ser)
  }
}

impl<'de> DeserializeV5<'de> for Upgrades {
  fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
    let val = u8::deserialize(de)?;

    Ok(Upgrades {
      speed: val & 7,
      shield: (val & (1 << 3)) != 0,
      inferno: (val & (1 << 4)) != 0,
    })
  }
}
