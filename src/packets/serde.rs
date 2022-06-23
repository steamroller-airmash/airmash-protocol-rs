use crate::types::VectorExt;
use crate::Vector2;

type FVec2 = Vector2;

#[derive(Serialize, Deserialize)]
#[serde(remote = "FVec2")]
pub(crate) struct VecRemote {
  #[serde(getter = "VecRemote::get_x")]
  x: f32,
  #[serde(getter = "VecRemote::get_y")]
  y: f32,
}

impl VecRemote {
  fn get_x(v: &FVec2) -> f32 {
    v.x
  }
  fn get_y(v: &FVec2) -> f32 {
    v.y
  }
}

impl From<VecRemote> for FVec2 {
  fn from(r: VecRemote) -> Self {
    FVec2::new(r.x, r.y)
  }
}

pub(crate) mod opt_vec {
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  use super::FVec2;
  use crate::types::VectorExt;

  #[derive(Serialize, Deserialize)]
  pub struct FakeVec {
    x: f32,
    y: f32,
  }

  impl From<FVec2> for FakeVec {
    fn from(v: FVec2) -> Self {
      Self { x: v.x, y: v.y }
    }
  }

  impl From<FakeVec> for FVec2 {
    fn from(v: FakeVec) -> Self {
      FVec2::new(v.x, v.y)
    }
  }

  pub(crate) fn serialize<S>(v: &Option<FVec2>, ser: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    v.map(FakeVec::from).serialize(ser)
  }

  pub(crate) fn deserialize<'de, D>(de: D) -> Result<Option<FVec2>, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(<Option<FakeVec>>::deserialize(de)?.map(FVec2::from))
  }
}
