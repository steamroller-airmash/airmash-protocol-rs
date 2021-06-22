use crate::types::Player;

#[derive(Clone, Debug)]
pub struct ChatSay {
	pub id: Player,
	pub text: String,
}
