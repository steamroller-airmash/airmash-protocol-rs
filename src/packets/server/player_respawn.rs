use crate::types::{Player, Position, Rotation, Upgrades};

/// Packet for when a player respawns.
#[derive(Copy, Clone, Debug)]
pub struct PlayerRespawn {
  pub id: Player,
  pub pos: Position,
  pub rot: Rotation,
  pub upgrades: Upgrades,
}
