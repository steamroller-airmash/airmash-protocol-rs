use crate::types::Player;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatPublic {
	pub id: Player,
	pub text: String,
}
