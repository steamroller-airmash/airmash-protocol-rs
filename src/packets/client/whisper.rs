use crate::types::Player;

/// Send a whisper to another player.
#[derive(Clone, Debug)]
pub struct Whisper {
	pub id: Player,
	pub text: String,
}
