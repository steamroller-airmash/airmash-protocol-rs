//! Packets that the server sends to the client.

use std::ops::{Deref, DerefMut};

use bstr::BString;

use crate::enums::*;
#[cfg(feature = "serde")]
use crate::packets::serde::{opt_vec, VecRemote};
use crate::types::*;

/// A player has said something in global chat.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatPublic {
  pub id: Player,
  pub text: BString,
}

/// A player has said something locally.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatSay {
  pub id: Player,
  pub text: BString,
}

/// A player has said something in team chat for the current team.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatTeam {
  pub id: Player,
  pub text: BString,
}

/// A player has been votemuted
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatVoteMutePassed {
  pub id: Player,
}

/// A player has whispered.
///
/// This only occurs if the current player is either the whisperer or the one
/// who originally sent the whisper message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatWhisper {
  pub from: Player,
  pub to: Player,
  pub text: BString,
}

/// Reply to a [`Command`](../client/struct.command.html).
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommandReply {
  // #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: CommandReplyType,
  pub text: BString,
}

/// Acknowledge successful receipt of a [`Backup`][0] packet.
///
/// [0]: ../client/struct.backup.html
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Backup;

/// TODO: Unknown why this is needed.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ack;

/// The current player has been votemuted.
///
/// This happens after enough players have sent a [`VoteMute`][0] packet to the
/// server.
///
/// [0]: ../client/struct.VoteMute.html
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatVoteMuted;

/// The client has carried out an invalid action, been ratelimited, or is
/// banned.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Error {
  pub error: ErrorType,
}

/// A predator has begun/stopped boosting
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventBoost {
  pub clock: u32,
  pub id: Player,
  pub boost: bool,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
  pub energy: Energy,
  pub energy_regen: EnergyRegen,
}

/// A player has run into a wall
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventBounce {
  pub clock: u32,
  pub id: Player,
  pub keystate: ServerKeyState,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
}

/// Event for when a player goes beyond the event horizon.
///
/// This indicates that the server will stop sending updates about this plane
/// until it comes back within the event horizon.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventLeaveHorizon {
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: LeaveHorizonType,
  /// This could be either a player or a mob
  pub id: u16,
}

/// A player has been repelled by a goliath.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventRepelPlayer {
  pub id: Player,
  pub keystate: ServerKeyState,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
  pub energy: Energy,
  pub energy_regen: EnergyRegen,
  pub health: Health,
  pub health_regen: HealthRegen,
}

/// A projectile has been repelled by a goliath
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventRepelMob {
  pub id: Mob,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: MobType,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub accel: Accel,
  pub max_speed: Speed,
}

/// Event triggered when something (player or missile) is deflected by a goliath
/// repel.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventRepel {
  pub clock: u32,
  pub id: Player,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
  pub energy: Energy,
  pub energy_regen: EnergyRegen,
  pub players: Vec<EventRepelPlayer>,
  pub mobs: Vec<EventRepelMob>,
}

/// A prowler has entered/exited stealth mode
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventStealth {
  pub id: Player,
  pub state: bool,
  pub energy: Energy,
  pub energy_regen: EnergyRegen,
}

/// Update the "Wall of Fire" in BTR
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameFirewall {
  /// This field is not used for anything by the client.
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: u8,
  pub status: FirewallStatus,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub radius: f32,
  pub speed: f32,
}

/// Update position of flag in CTF
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameFlag {
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: FlagUpdateType,
  pub flag: u8,
  pub id: Option<Player>,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  /// Blue team score
  pub blueteam: u8,
  /// Red team score
  pub redteam: u8,
}

/// Info on the number of players currently alive
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GamePlayersAlive {
  pub players: u16,
}

/// Update which player the client is spectating.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameSpectate {
  pub id: Player,
}

/// Initial data passed in for a player when the server starts.
///
/// This is an element of the `players` array within the [`Login`] packet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LoginPlayer {
  pub id: Player,
  pub status: PlayerStatus,
  pub level: Level,
  pub name: BString,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: PlaneType,
  pub team: Team,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  pub flag: FlagCode,
  pub upgrades: Upgrades,
}

/// Initial Login packet sent to the server
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Login {
  pub success: bool,
  pub id: Player,
  pub team: Team,
  pub clock: u32,
  pub token: BString,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: GameType,
  pub room: BString,
  pub players: Vec<LoginPlayer>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LoginBot {
  pub id: Player,
}

/// Upgraded Login packet introduced in <https://github.com/wight-airmash/ab-protocol>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub struct Login2 {
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub login: Login,
  #[cfg_attr(feature = "serde", serde(rename = "serverConfiguration"))]
  pub config: BString,
  pub bots: Vec<LoginBot>,
}

/// A missile despawned with an explosion. This is used when a missile collides
/// with a mountain to generate an explosion client-side.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MobDespawnCoords {
  pub id: Mob,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: MobType,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
}

/// A mob despawned.
///
/// This is used when a powerup despawns and when a missile despawns without
/// hitting anything. It does not cause an explosion to be shown at the
/// location.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MobDespawn {
  pub id: Mob,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: DespawnType,
}

/// Update for an immobile mob.
///
/// This is sent when a powerup is enters a player's view radius.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MobUpdateStationary {
  pub id: Mob,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: MobType,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
}

/// Update for a mobile missile-type mob.
///
/// This is sent when a mob enters a player's view radius or something changes
/// about its state that needs to be communicated to the client.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MobUpdate {
  pub clock: u32,
  pub id: Mob,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: MobType,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub accel: Accel,
  pub max_speed: Speed,
}

/// MobUpdate but extended with an extra ownerId field as present ab-protocol.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MobUpdate2 {
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub update: MobUpdate,
  #[cfg_attr(feature = "serde", serde(flatten))]
  #[cfg_attr(feature = "serde", serde(rename = "ownerId"))]
  pub owner: Player,
}

/// Resulting ping data sent back from the server.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PingResult {
  pub ping: u16,
  #[cfg_attr(feature = "serde", serde(rename = "playersTotal"))]
  pub players_total: u32,
  #[cfg_attr(feature = "serde", serde(rename = "playersGame"))]
  pub players_game: u32,
}

/// A ping request by the server.
///
/// All clients must respond with a [`Pong`](../client/struct.pong.html) with
/// `num` set to the same value as this packet. If a client does not do this,
/// the client will be disconnected by the server.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ping {
  pub clock: u32,
  pub num: u32,
}

/// Data on a projectile fired by a plane.
///
/// This is used in the `projectiles` array of the [`PlayerFire`] packet.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerFireProjectile {
  pub id: Mob,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: MobType,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub accel: Accel,
  pub max_speed: Speed,
}

/// Packet for whan a player fires missiles.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerFire {
  pub clock: u32,
  pub id: Player,
  pub energy: Energy,
  pub energy_regen: EnergyRegen,
  pub projectiles: Vec<PlayerFireProjectile>,
}

/// Packet for when a player changes their flag.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerFlag {
  pub id: Player,
  pub flag: FlagCode,
}

/// Data on a player that has been hit by a shot fired by another player.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerHitPlayer {
  pub id: Player,
  pub health: Health,
  pub health_regen: HealthRegen,
}

/// Event for when players have been hit by a missile.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerHit {
  pub id: Mob,
  // #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: MobType,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub owner: Player,
  pub players: Vec<PlayerHitPlayer>,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerKill {
  pub id: Player,
  pub killer: Option<Player>,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
}

/// Packet for when a player leaves.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerLeave {
  pub id: Player,
}

/// Assign a level to a player.
///
/// Either the player levelled up, or the server is updating their level for all
/// clients.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerLevel {
  pub id: Player,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: PlayerLevelType,
  pub level: Level,
}

/// Data for a newly-joined player.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerNew {
  pub id: Player,
  pub status: PlayerStatus,
  pub name: BString,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: PlaneType,
  pub team: Team,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  pub flag: FlagCode,
  pub upgrades: Upgrades,
}

/// The current player picked up a powerup.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerPowerup {
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: PowerupType,
  /// Lifetime of the powerup, in milliseconds.
  pub duration: u32,
}

/// Packet for when a player respawns.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerRespawn {
  pub id: Player,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  pub upgrades: Upgrades,
}

/// Details about a player that has switched teams.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerReteamPlayer {
  pub id: Player,
  pub team: Team,
}

/// Packet for when players change teams
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerReteam {
  /// List of players that have changed teams.
  pub players: Vec<PlayerReteamPlayer>,
}

/// A player has switched planes.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerType {
  pub id: Player,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: PlaneType,
}

/// Movement update for a player.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerUpdate {
  pub clock: u32,
  pub id: Player,
  pub keystate: ServerKeyState,
  pub upgrades: Upgrades,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub pos: Position,
  pub rot: Rotation,
  #[cfg_attr(feature = "serde", serde(with = "VecRemote"))]
  pub speed: Velocity,
}

/// A player has upgraded themselves.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlayerUpgrade {
  pub upgrades: u16,
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: UpgradeType,
  pub speed: u8,
  pub defense: u8,
  pub energy: u8,
  pub missile: u8,
}

/// Leaderboard data, part of the [`ScoreBoard`] packet.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreBoardData {
  pub id: Player,
  pub score: Score,
  pub level: Level,
}

/// Low-res player positions, part of the [`ScoreBoard`] packet.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreBoardRanking {
  pub id: Player,
  #[cfg_attr(feature = "serde", serde(with = "opt_vec"))]
  pub pos: Option<Position>,
}

/// Leaderboard + Global player positions.
///
/// This is sent every 5 seconds by the server and is used by the client to
/// update the leaderboard and minimap.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreBoard {
  pub data: Vec<ScoreBoardData>,
  pub rankings: Vec<ScoreBoardRanking>,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreUpdate {
  pub id: Player,
  pub score: Score,
  pub earnings: Score,
  /// The number of unused upgrades that the player has.
  pub upgrades: u16,
  pub total_kills: u32,
  pub total_deaths: u32,
}

/// End of game packet for CTF and BTR.
///
/// # CTF
/// In CTF, the data of this packet contains a JSON string with 3 fields.
///
/// - `w`: The id of the winning team.
/// - `b`: The bounty given to each player
/// of the winning team.
/// - `t`: The time (in seconds) that the banner should remain on screen before
///   closing (unless closed by the player.)
///
/// # BTR
/// In BTR, the data of this packet contains a JSON string with 5 fields.
///
/// - `p`: The name of the winning player.
/// - `f`: The flag id of the winning player.
/// - `b`: The bounty given to the winning player.
/// - `k`: The number of kills that the winning player has.
/// - `t`: The time (in seconds) that the banner should remain on the screen
///   before closing (unless closed by the player.)
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServerCustom {
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: ServerCustomType,
  pub data: BString,
}

/// Server banned message
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServerMessage {
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  pub ty: ServerMessageType,
  pub duration: u32,
  pub text: BString,
}

/// Per-player data for detailed (tab) menu in BTR.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedBTREntry {
  pub id: Player,
  pub level: Level,
  pub alive: bool,
  pub wins: u16,
  pub score: Score,
  pub kills: u16,
  pub deaths: u16,
  pub damage: f32,
  pub ping: u16,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedBTR {
  pub scores: Vec<ScoreDetailedBTREntry>,
}

/// Per-player data for detailed (tab) menu in CTF.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedCTF {
  pub scores: Vec<ScoreDetailedCTFEntry>,
}

/// Per-player data for detailed (tab) menu in FFA.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreDetailedFFA {
  pub scores: Vec<ScoreDetailedFFAEntry>,
}

//===================================================================
// Extra trait implementations

impl Deref for Login2 {
  type Target = Login;

  fn deref(&self) -> &Self::Target {
    &self.login
  }
}

impl DerefMut for Login2 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.login
  }
}

impl Deref for MobUpdate2 {
  type Target = MobUpdate;

  fn deref(&self) -> &Self::Target {
    &self.update
  }
}

impl DerefMut for MobUpdate2 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.update
  }
}
