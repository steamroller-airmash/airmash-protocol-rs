
use std::result;
use serde_am::*;
use error::Error;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum KeyCode {
    Up,
    Down,
    Left,
    Right,
    Fire,
    Special
}

impl KeyCode {
    pub fn from_u8(code: u8) -> result::Result<KeyCode, u8> {
        match code {
            1 => Ok(KeyCode::Up),
            2 => Ok(KeyCode::Down),
            3 => Ok(KeyCode::Left),
            4 => Ok(KeyCode::Right),
            5 => Ok(KeyCode::Fire),
            6 => Ok(KeyCode::Special),
            _ => Err(code)
        }
    }
    pub fn to_u8(self) -> u8 {
        match self {
            KeyCode::Up => 1,
            KeyCode::Down => 2,
            KeyCode::Left => 3,
            KeyCode::Right => 4,
            KeyCode::Fire => 5,
            KeyCode::Special => 6
        }
    }
}

impl Serialize for KeyCode {
    fn serialize(&self, ser: &mut Serializer) -> Result<()> {
        ser.serialize_u8(self.to_u8())
    }
}

impl<'de> Deserialize<'de> for KeyCode {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<KeyCode> {
        match KeyCode::from_u8(de.deserialize_u8()?) {
            Ok(code) => Ok(code),
            Err(e) => Err(Error::InvalidKeyCode(e))
        }
    }
}



