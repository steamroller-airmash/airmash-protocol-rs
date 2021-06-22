use crate::types::{Level, Player, Score};

/// Per-player data for detailed (tab) menu in CTF.
#[derive(Copy, Clone, Debug)]
pub struct ScoreDetailedCTFEntry {
	pub id: Player,
	pub level: Level,
	pub captures: u16,
	pub score: Score,
	pub kills: u16,
	pub deaths: u16,
	pub damage: f32,
	pub ping: u16,
}

/// Detailed score menu (tab) data for CTF.
#[derive(Clone, Debug)]
pub struct ScoreDetailedCTF {
	pub scores: Vec<ScoreDetailedCTFEntry>,
}
