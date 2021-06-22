use crate::types::{Player, Position};

#[derive(Copy, Clone, Debug)]
pub struct PlayerKill {
  pub id: Player,
  pub killer: Option<Player>,
  pub pos: Position,
}
