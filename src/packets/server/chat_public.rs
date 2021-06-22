use crate::types::Player;

#[derive(Clone, Debug)]
pub struct ChatPublic {
	pub id: Player,
	pub text: String,
}
