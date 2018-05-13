
use serde_am::*;

#[derive(Debug, Copy, Clone)]
pub enum KeyState {
    Pressed,
    Released
}

impl KeyState {
    pub fn from_bool(v: bool) -> KeyState {
        if v { KeyState::Pressed } else { KeyState::Released }
    }
    pub fn to_bool(self) -> bool {
        match self {
            KeyState::Pressed => true,
            KeyState::Released => false
        }
    }
}

impl Serialize for KeyState {
    fn serialize(&self, ser: &mut Serializer) -> Result<()> {
        self.to_bool().serialize(ser)
    }
}
impl<'de> Deserialize<'de> for KeyState {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
        Ok(Self::from_bool(bool::deserialize(de)?))
    }
}
