use crate::types::{Level, Player, Position, Score};

/// Leaderboard data, part of the [`ScoreBoard`] packet.
#[derive(Copy, Clone, Debug)]
pub struct ScoreBoardData {
  pub id: Player,
  pub score: Score,
  pub level: Level,
}

/// Low-res player positions, part of the
/// [`ScoreBoard`] packet.
#[derive(Copy, Clone, Debug)]
pub struct ScoreBoardRanking {
  pub id: Player,
  pub pos: Option<Position>,
}

/// Leaderboard + Global player positions
///
/// This is sent every 5 seconds by the
/// server and is used by the client to
/// update the leaderboard and minimap.
#[derive(Clone, Debug)]
pub struct ScoreBoard {
  pub data: Vec<ScoreBoardData>,
  pub rankings: Vec<ScoreBoardRanking>,
}
