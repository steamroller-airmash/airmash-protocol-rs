use crate::enums::MobType;
use crate::types::{Accel, Mob, Position, Speed, Velocity};

#[derive(Copy, Clone, Debug)]
pub struct MobUpdate {
  pub clock: u32,
  pub id: Mob,
  // #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: MobType,
  pub pos: Position,
  pub speed: Velocity,
  pub accel: Accel,
  pub max_speed: Speed,
}
