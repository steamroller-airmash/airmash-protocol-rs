
use serde_am::*;
use datatypes::{KeyCode, KeyState};

use bit_field::BitField;

/// Key state bitfield for PlayerUpdate packet
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
#[cfg_attr(features="serde", derive(Serialize, Deserialize))]
pub struct ServerKeyState(pub u8);

fn get_index(key: KeyCode) -> usize {
	match key {
		KeyCode::Special => 5,
		KeyCode::Fire    => 4,
		KeyCode::Right   => 3,
		KeyCode::Left    => 2,
		KeyCode::Down    => 1,
		KeyCode::Up      => 0,
	}
}

impl ServerKeyState {
	pub fn get(&self, key: KeyCode) -> KeyState {
		let bit = self.0.get_bit(get_index(key));
		if bit {
			KeyState::Pressed
		}
		else {
			KeyState::Released
		}
	}

	pub fn set(&mut self, key: KeyCode, state: KeyState) {
		let bit = match state { 
			KeyState::Pressed => true, 
			KeyState::Released => false
		};

		self.0.set_bit(get_index(key), bit);
	}
}

impl Serialize for ServerKeyState {
	fn serialize(&self, ser: &mut Serializer) -> Result<()> {
		ser.serialize_u8(self.0)
	}
}
impl<'de> Deserialize<'de> for ServerKeyState {
	fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
		Ok(ServerKeyState(de.deserialize_u8()?))
	}
}
