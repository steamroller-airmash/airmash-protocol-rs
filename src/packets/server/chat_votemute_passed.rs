use crate::types::Player;

/// A player has been votemuted
#[derive(Copy, Clone, Debug)]
pub struct ChatVoteMutePassed {
	pub id: Player,
}
