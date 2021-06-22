use crate::enums::MobType;
use crate::types::{Mob, Position};

/// A missile despawned with an explosion
/// This is used when a missile
/// collides with a mountain to
/// generate an explosion client-side
#[derive(Copy, Clone, Debug)]
pub struct MobDespawnCoords {
	pub id: Mob,
	// #[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: MobType,
	pub pos: Position,
}
