use crate::types::Player;

#[derive(Clone, Debug)]
pub struct ChatTeam {
	pub id: Player,
	pub text: String,
}
