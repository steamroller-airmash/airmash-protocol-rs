
use serde_am::*;

/// The current state of a key.
/// 
/// This may be converted back to a `bool`
/// in future versions if it turns out not
/// to be worth it.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum KeyState {
    Pressed,
    Released
}

impl KeyState {
    /// Get the keystate from the boolean value
    /// associated with it.
    pub fn from_bool(v: bool) -> KeyState {
        if v { KeyState::Pressed } else { KeyState::Released }
    }
    /// Convert the keystate back to the associated
    /// boolean value.
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
