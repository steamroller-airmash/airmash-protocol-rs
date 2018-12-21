#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

impl_try_from_enum! {
	/// Details on how the mob despawned. (i.e. whether
	/// it's lifetime ended or it collided with some
	/// other object)
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum DespawnType {
		LifetimeEnded = 0,
		Collided = 1,
	}
}
