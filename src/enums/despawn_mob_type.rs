#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};

use super::MobType;

impl_try_from_enum! {
	/// The type of a mob that despawned.
	///
	/// This is less specific than the regular
	/// [`MobType`].
	///
	/// [`MobType`]: ../enum.MobType.html
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum DespawnMobType {
		Missile = 0,
		// Also includes upgrades
		Powerup = 1,
	}
}

impl From<MobType> for DespawnMobType {
	fn from(ty: MobType) -> DespawnMobType {
		use self::MobType::*;

		match ty {
			PredatorMissile => DespawnMobType::Missile,
			GoliathMissile => DespawnMobType::Missile,
			MohawkMissile => DespawnMobType::Missile,
			Upgrade => DespawnMobType::Powerup,
			TornadoSingleMissile => DespawnMobType::Missile,
			TornadoTripleMissile => DespawnMobType::Missile,
			ProwlerMissile => DespawnMobType::Missile,
			Shield => DespawnMobType::Powerup,
			Inferno => DespawnMobType::Powerup,
		}
	}
}
