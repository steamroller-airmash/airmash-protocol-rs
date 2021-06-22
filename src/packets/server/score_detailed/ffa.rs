use crate::types::{Level, Player, Score};

/// Per-player data for detailed (tab) menu in FFA.
#[derive(Copy, Clone, Debug)]
pub struct ScoreDetailedFFAEntry {
  pub id: Player,
  pub level: Level,
  pub score: Score,
  pub kills: u16,
  pub deaths: u16,
  pub damage: f32,
  pub ping: u16,
}

/// Detailed score menu (tab) data for FFA.
#[derive(Clone, Debug)]
pub struct ScoreDetailedFFA {
  pub scores: Vec<ScoreDetailedFFAEntry>,
}
