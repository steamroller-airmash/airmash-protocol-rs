mod units;

mod server_key_state;
mod upgrades;

pub use dimensioned::Sqrt;

pub use self::units::*;
pub use nalgebra::Vector2;

pub use self::server_key_state::ServerKeyState;
pub use self::upgrades::Upgrades;

pub type Player = u16;
pub type Mob = u16;
pub type Team = u16;
pub type Flag = u16;
pub type Level = u8;
pub type Score = u32;
