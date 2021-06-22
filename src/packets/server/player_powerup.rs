use crate::enums::PowerupType;

/// The current player picked up a powerup.
#[derive(Copy, Clone, Debug)]
pub struct PlayerPowerup {
  // #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: PowerupType,
  // Maybe make this a Duration?
  /// Lifetime of the powerup, in milliseconds.
  pub duration: u32,
}
