#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

impl_try_from_enum! {
	/// Used to indicate the type of plane
	/// that the packet refers to.
	///
	/// Used in:
	/// - TODO
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum PlaneType {
		Predator = 1,
		Goliath = 2,
		Mohawk = 3,
		Tornado = 4,
		Prowler = 5,
	}
}

impl Default for PlaneType {
	fn default() -> Self {
		PlaneType::Predator
	}
}
