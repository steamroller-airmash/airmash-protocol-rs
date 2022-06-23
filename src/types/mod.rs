mod units;

mod server_key_state;
mod upgrades;

pub use self::units::*;
pub(crate) type Vector2 = mint::Vector2<f32>;

pub use self::server_key_state::ServerKeyState;
pub use self::upgrades::Upgrades;

pub type Player = u16;
pub type Mob = u16;
pub type Team = u16;
pub type Flag = u16;
pub type Level = u8;
pub type Score = u32;

pub(crate) trait VectorExt {
  fn new(x: f32, y: f32) -> Self;
}

impl VectorExt for Vector2 {
  fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}
