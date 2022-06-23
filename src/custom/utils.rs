pub(crate) mod flag_code {
  use serde::*;

  use crate::enums::FlagCode;

  pub fn serialize<S>(&flag: &FlagCode, s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    s.serialize_u16(flag.into())
  }

  pub fn deserialize<'de, D>(de: D) -> Result<FlagCode, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(u16::deserialize(de)?.into())
  }
}

pub(crate) mod duration {
  use std::time::Duration;

  use serde::*;

  pub fn serialize<S>(duration: &Duration, s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    s.serialize_u64(duration.as_secs())
  }

  pub fn deserialize<'de, D>(de: D) -> Result<Duration, D::Error>
  where
    D: Deserializer<'de>,
  {
    Ok(Duration::from_secs(u64::deserialize(de)?))
  }
}
