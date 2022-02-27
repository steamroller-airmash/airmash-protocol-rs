use crate::enums::{FlagCode, PlaneType, PlayerStatus};
use crate::types::{Player, Position, Rotation, Team, Upgrades};

/// Data for a newly-joined player.
#[derive(Clone, Debug)]
pub struct PlayerNew {
	pub id: Player,
	pub status: PlayerStatus,
	pub name: String,
	// #[cfg_attr(feature = "serde", serde(rename = "type"))]
	pub ty: PlaneType,
	pub team: Team,
	pub pos: Position,
	pub rot: Rotation,
	pub flag: FlagCode,
	pub upgrades: Upgrades,
}
