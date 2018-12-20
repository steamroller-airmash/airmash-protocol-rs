#![allow(deprecated)]

#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

impl_try_from_enum! {
	/// All upgrade types.
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum UpgradeType {
		/// This seems to be sent by the official server when a
		/// player leaves. Packets with this value are ignored
		/// by the client, so they don't seem to affect gameplay
		/// at all.
		None = 0,
		Speed = 1,
		Defense = 2,
		Energy = 3,
		Missile = 4,
	}
}
