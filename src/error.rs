//! All error types for this crate.

use std::num::TryFromIntError;

/// Attempted to convert an enum from a value but
/// the value didn't map to any possible enum value.
pub struct EnumValueOutOfRangeError<T>(pub T);

// Can't get rid of this since it would be a breaking change.
// TODO: When the major version is bumped, remove this.
#[doc(hidden)]
#[deprecated(note = "This isn't used anywhere in this crate")]
pub struct EntityIdOutOfRangeError;
#[allow(deprecated)]
impl From<TryFromIntError> for EntityIdOutOfRangeError {
	fn from(_: TryFromIntError) -> Self {
		Self {}
	}
}
