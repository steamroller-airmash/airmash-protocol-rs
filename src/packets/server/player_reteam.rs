use crate::types::{Player, Team};

/// Details about a player that has switched teams.
#[derive(Copy, Clone, Debug)]
pub struct PlayerReteamPlayer {
	pub id: Player,
	pub team: Team,
}

/// Packet for when players change teams
#[derive(Clone, Debug)]
pub struct PlayerReteam {
	/// List of players that have changed teams.
	pub players: Vec<PlayerReteamPlayer>,
}
