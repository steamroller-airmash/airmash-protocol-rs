use crate::types::{Player, Position, Rotation, ServerKeyState, Upgrades, Velocity};

/// Movement update for a player.
#[derive(Copy, Clone, Debug)]
pub struct PlayerUpdate {
	pub clock: u32,
	pub id: Player,
	pub keystate: ServerKeyState,
	pub upgrades: Upgrades,
	pub pos: Position,
	pub rot: Rotation,
	pub speed: Velocity,
}
