use crate::types::Player;

/// Vote to mute a player
#[derive(Copy, Clone, Debug)]
pub struct VoteMute {
	pub id: Player,
}
