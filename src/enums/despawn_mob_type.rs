#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

/// Details on how the mob despawned. (i.e. whether
/// it's lifetime ended or it collided with some
/// other object)
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Primitive)]
#[cfg_attr(feature = "specs", derive(Component))]
pub enum DespawnType {
	LifetimeEnded = 0,
	Collided = 1,
}

impl_try_from2!(DespawnType);
