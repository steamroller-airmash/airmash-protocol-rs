use crate::enums::ErrorType;

/// The client has carried out an invalid action,
/// been ratelimited, or is banned.
#[derive(Copy, Clone, Debug)]
pub struct Error {
	pub error: ErrorType,
}
