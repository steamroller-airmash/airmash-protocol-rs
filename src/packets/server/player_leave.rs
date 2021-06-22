use crate::types::Player;

/// Packet for when a player leaves.
#[derive(Copy, Clone, Debug)]
pub struct PlayerLeave {
  pub id: Player,
}
