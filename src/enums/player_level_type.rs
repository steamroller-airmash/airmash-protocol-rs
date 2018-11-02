#[cfg(feature = "specs")]
use specs::{DenseVecStorage, Component};

impl_try_from_enum! {
	/// Indicate whether a player levelled up, or has
	/// just logged in and their level is being communicated
	/// to the client.
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum PlayerLevelType {
		Login = 0,
		LevelUp = 1,
	}
}

impl Default for PlayerLevelType {
	fn default() -> Self {
		PlayerLevelType::Login
	}
}
