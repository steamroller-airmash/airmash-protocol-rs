use crate::server::*;
use crate::v5::{AirmashDeserializerV5, AirmashSerializerV5, DeserializeV5, Result, SerializeV5};
use crate::ServerPacket;

decl_serde! {
  struct LoginPlayer {
    id,
    status,
    level,
    name => { serialize_text_small, deserialize_text_small },
    ty,
    team,
    pos => { serialize_pos, deserialize_pos },
    rot => { serialize_rot, deserialize_rot },
    flag,
    upgrades
  }

  struct Login {
    success,
    id,
    team,
    clock,
    token => { serialize_text_small, deserialize_text_small },
    ty,
    room => { serialize_text_small, deserialize_text_small },
    players => { serialize_array_large, deserialize_array_large },
  }

  struct Ping {
    clock,
    num
  }

  struct PingResult {
    ping,
    players_total,
    players_game,
  }

  struct CommandReply {
    ty,
    text => { serialize_text_large, deserialize_text_large }
  }

  struct PlayerNew {
    id,
    status,
    name => { serialize_text_small, deserialize_text_small },
    ty,
    team,
    pos => { serialize_pos, deserialize_pos },
    rot => { serialize_rot, deserialize_rot },
    flag,
    upgrades,
  }

  struct PlayerLeave {
    id
  }

  struct PlayerUpdate {
    clock,
    id,
    keystate,
    upgrades,
    pos => { serialize_pos24, deserialize_pos24 },
    rot => { serialize_rot, deserialize_rot },
    speed => { serialize_speed, deserialize_speed }
  }

  struct PlayerFireProjectile {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    speed => { serialize_vel, deserialize_vel },
    accel => { serialize_accel, deserialize_accel },
    max_speed => { serialize_speed, deserialize_speed }
  }

  // Note: Need to implement this one manually
  struct PlayerFire {
    clock,
    id,
    energy => { serialize_energy, deserialize_energy },
    energy_regen => { serialize_regen, deserialize_regen },
    projectiles => { serialize_array_small, deserialize_array_small },
  }

  struct PlayerRespawn {
    id,
    pos => { serialize_pos24, deserialize_pos24 },
    rot => { serialize_rot, deserialize_rot },
    upgrades
  }

  struct PlayerFlag {
    id,
    flag
  }

  struct PlayerLevel {
    id,
    ty,
    level
  }

  struct PlayerHitPlayer {
    id,
    health => { serialize_energy, deserialize_energy },
    health_regen => { serialize_regen, deserialize_regen }
  }

  // TODO: Need manual fixup
  struct PlayerHit {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    owner,
    players => { serialize_array_small, deserialize_array_small }
  }

  // TODO: Needs to be done manually as well
  struct PlayerKill {
    id,
    killer,
    pos => { serialize_pos, deserialize_pos }
  }

  struct PlayerUpgrade {
    upgrades,
    ty,
    speed,
    defense,
    energy,
    missile
  }

  struct PlayerType {
    id,
    ty
  }

  struct PlayerPowerup {
    ty,
    duration
  }

  struct PlayerReteamPlayer {
    id,
    team
  }

  struct PlayerReteam {
    players => { serialize_array_large, deserialize_array_large }
  }

  struct GameFlag {
    ty,
    flag,
    id,
    pos => { serialize_pos24, deserialize_pos24 },
    blueteam,
    redteam
  }

  struct GameSpectate {
    id
  }

  struct GamePlayersAlive {
    players
  }

  struct GameFirewall {
    ty,
    status,
    pos => { serialize_pos, deserialize_pos },
    radius,
    speed
  }

  struct EventRepelPlayer {
    id,
    keystate,
    pos => { serialize_pos, deserialize_pos },
    rot => { serialize_rot, deserialize_rot },
    speed => { serialize_vel, deserialize_vel },
    energy => { serialize_energy, deserialize_energy },
    energy_regen => { serialize_regen, deserialize_regen },
    health => { serialize_energy, deserialize_energy },
    health_regen => { serialize_regen, deserialize_regen }
  }

  struct EventRepelMob {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    speed => { serialize_vel, deserialize_vel },
    accel => { serialize_accel, deserialize_accel },
    max_speed => { serialize_speed, deserialize_speed }
  }

  struct EventRepel {
    clock,
    id,
    pos => { serialize_pos, deserialize_pos },
    rot => { serialize_rot, deserialize_rot },
    speed => { serialize_vel, deserialize_vel },
    energy => { serialize_energy, deserialize_energy },
    energy_regen => { serialize_regen, deserialize_regen },
    players => { serialize_array_small, deserialize_array_small },
    mobs => { serialize_array_small, deserialize_array_small }
  }

  struct EventBoost {
    clock,
    id,
    boost,
    pos => { serialize_pos24, deserialize_pos24 },
    rot => { serialize_rot, deserialize_rot },
    speed => { serialize_vel, deserialize_vel },
    energy => { serialize_energy, deserialize_energy },
    energy_regen => { serialize_regen, deserialize_regen },
  }

  struct EventBounce {
    clock,
    id,
    keystate,
    pos => { serialize_pos24, deserialize_pos24 },
    rot => { serialize_rot, deserialize_rot },
    speed => { serialize_vel, deserialize_vel }
  }

  struct EventStealth {
    id,
    state,
    energy => { serialize_energy, deserialize_energy },
    energy_regen => { serialize_regen, deserialize_regen }
  }

  struct EventLeaveHorizon {
    ty,
    id
  }

  struct MobUpdate {
    clock,
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    speed => { serialize_vel, deserialize_vel },
    accel => { serialize_accel, deserialize_accel },
    max_speed => { serialize_speed, deserialize_speed }
  }

  struct MobUpdateStationary {
    id,
    ty,
    pos => { serialize_pos_f32, deserialize_pos_f32 }
  }

  struct MobDespawn {
    id,
    ty
  }

  struct MobDespawnCoords {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos }
  }

  struct ScoreUpdate {
    id,
    score,
    earnings,
    upgrades,
    total_kills,
    total_deaths
  }

  struct ScoreBoardData {
    id,
    score,
    level
  }

  struct ScoreBoardRanking {
    id,
    pos => { serialize_low_res_pos, deserialize_low_res_pos }
  }

  struct ScoreDetailedFFAEntry {
    id,
    level,
    score,
    kills,
    deaths,
    damage,
    ping
  }

  struct ScoreDetailedFFA {
    scores => { serialize_array_large, deserialize_array_large }
  }

  struct ScoreDetailedCTFEntry {
    id,
    level,
    captures,
    score,
    kills,
    deaths,
    damage,
    ping
  }

  struct ScoreDetailedCTF {
    scores => { serialize_array_large, deserialize_array_large }
  }

  struct ScoreDetailedBTREntry {
    id,
    level,
    alive,
    wins,
    score,
    kills,
    deaths,
    damage,
    ping
  }

  struct ScoreDetailedBTR {
    scores => { serialize_array_large, deserialize_array_large }
  }

  struct ChatTeam {
    id,
    text => { serialize_text_small, deserialize_text_small }
  }

  struct ChatPublic {
    id,
    text => { serialize_text_small, deserialize_text_small }
  }

  struct ChatSay {
    id,
    text => { serialize_text_small, deserialize_text_small }
  }

  struct ChatWhisper {
    from,
    to,
    text => { serialize_text_small, deserialize_text_small }
  }

  struct ChatVoteMutePassed {
    id
  }

  struct ServerMessage {
    ty,
    duration,
    text => { serialize_text_large, deserialize_text_large }
  }

  struct ServerCustom {
    ty,
    data => { serialize_text_large, deserialize_text_large }
  }

  struct Error {
    error
  }
}

decl_consts! {
  const Login = 0;
  const Backup = 1;
  const Ping = 5;
  const PingResult = 6;
  const Ack = 7;
  const Error = 8;
  const CommandReply = 9;
  const PlayerNew = 10;
  const PlayerLeave = 11;
  const PlayerUpdate = 12;
  const PlayerFire = 13;
  const PlayerHit = 14;
  const PlayerRespawn = 15;
  const PlayerFlag = 16;
  const PlayerKill = 17;
  const PlayerUpgrade = 18;
  const PlayerType = 19;
  const PlayerPowerup = 20;
  const PlayerLevel = 21;
  const PlayerReteam = 22;
  const GameFlag = 30;
  const GameSpectate = 31;
  const GamePlayersAlive = 32;
  const GameFirewall = 33;
  const EventRepel = 40;
  const EventBoost = 41;
  const EventBounce = 42;
  const EventStealth = 43;
  const EventLeaveHorizon = 44;
  const MobUpdate = 60;
  const MobUpdateStationary = 61;
  const MobDespawn = 62;
  const MobDespawnCoords = 63;
  const ChatPublic = 70;
  const ChatTeam = 71;
  const ChatSay = 72;
  const ChatWhisper = 73;
  const ChatVoteMutePassed = 78;
  const ChatVoteMuted = 79;
  const ScoreUpdate = 80;
  const ScoreBoard = 81;
  const ScoreDetailedFFA = 82;
  const ScoreDetailedCTF = 83;
  const ScoreDetailedBTR = 84;
  const ServerMessage = 90;
  const ServerCustom = 91;
}

packet_serde! {
  enum ServerPacket {
    Login(x),
    Backup,
    Ping(x),
    PingResult(x),
    Ack,
    Error(x),
    CommandReply(x),
    PlayerNew(x),
    PlayerLeave(x),
    PlayerUpdate(x),
    PlayerFire(x),
    PlayerRespawn(x),
    PlayerFlag(x),
    PlayerHit(x),
    PlayerKill(x),
    PlayerUpgrade(x),
    PlayerType(x),
    PlayerPowerup(x),
    PlayerLevel(x),
    PlayerReteam(x),
    GameFlag(x),
    GameSpectate(x),
    GamePlayersAlive(x),
    GameFirewall(x),
    EventRepel(x),
    EventBoost(x),
    EventBounce(x),
    EventStealth(x),
    EventLeaveHorizon(x),
    MobUpdate(x),
    MobUpdateStationary(x),
    MobDespawn(x),
    MobDespawnCoords(x),
    ScoreUpdate(x),
    ScoreBoard(x),
    ScoreDetailedFFA(x),
    ScoreDetailedCTF(x),
    ScoreDetailedBTR(x),
    ChatTeam(x),
    ChatPublic(x),
    ChatSay(x),
    ChatWhisper(x),
    ChatVoteMutePassed(x),
    ChatVoteMuted,
    ServerMessage(x),
    ServerCustom(x)
  }
}
