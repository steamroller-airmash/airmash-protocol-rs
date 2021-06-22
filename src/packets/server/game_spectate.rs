use crate::types::Player;

/// Update which player the client is spectating.
#[derive(Copy, Clone, Debug)]
pub struct GameSpectate {
	pub id: Player,
}
