//! All error types for this crate.

use std::num::TryFromIntError;

pub struct EntityIdOutOfRangeError;
pub struct EnumValueOutOfRangeError<T>(pub T);

impl From<TryFromIntError> for EntityIdOutOfRangeError {
	fn from(_: TryFromIntError) -> Self {
		Self {}
	}
}
