use crate::enums::MobType;
use crate::types::{Mob, Position};

/// Update for powerups
#[derive(Copy, Clone, Debug)]
pub struct MobUpdateStationary {
	pub id: Mob,
	// #[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: MobType,
	pub pos: Position,
}
