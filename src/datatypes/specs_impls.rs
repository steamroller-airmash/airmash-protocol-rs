
extern crate specs;

use self::specs::*;
use datatypes::*;

impl Component for FlagCode {
	type Storage = DenseVecStorage<FlagCode>;
}

impl Component for PlaneType {
	type Storage = DenseVecStorage<PlaneType>;
}

impl Component for PlayerStatus {
	type Storage = DenseVecStorage<Self>;
}

impl Component for MobType {
	type Storage = DenseVecStorage<Self>;
}
