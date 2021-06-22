use crate::Team;
use std::time::Duration;

use super::utils::*;

/// Serde serialization declaration for CTF [`ServerCustom`][0]
/// data.
///
/// This struct will serialize from/deserialize to the JSON
/// representation used in the `data` field of `ServerCustom`.
///
/// # Serialization Notes
/// - `duration` is only encoded at the resolution of seconds.
///
/// [0]: ../packets/client/struct.ServerCustom.html
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CTFData {
	#[cfg_attr(feature = "serde", serde(rename = "w"))]
	pub winner: Team,
	#[cfg_attr(feature = "serde", serde(rename = "b"))]
	pub bounty: u32,
	#[cfg_attr(feature = "serde", serde(rename = "t"))]
	#[cfg_attr(feature = "serde", serde(with = "duration"))]
	pub duration: Duration,
}
