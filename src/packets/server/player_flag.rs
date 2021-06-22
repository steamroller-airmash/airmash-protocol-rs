use crate::enums::FlagCode;
use crate::types::Player;

/// Packet for when a player changes their flag.
#[derive(Copy, Clone, Debug)]
pub struct PlayerFlag {
  pub id: Player,
  pub flag: FlagCode,
}
