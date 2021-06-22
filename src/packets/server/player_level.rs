use crate::enums::PlayerLevelType;
use crate::types::{Level, Player};

/// Assign a level to a player. Either the player levelled up, or the server is
/// updating their level for all clients.
#[derive(Copy, Clone, Debug)]
pub struct PlayerLevel {
  pub id: Player,
  // #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: PlayerLevelType,
  pub level: Level,
}
