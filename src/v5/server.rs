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
}

decl_serde! {
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
}

decl_serde! {
  struct LoginBot {
    id
  }
}

decl_serde! {
  struct Login2 {
    login,
    config => { serialize_text_large, deserialize_text_large },
    bots => { serialize_array_large, deserialize_array_large },
  }
}

decl_serde! {
  struct Ping {
    clock,
    num
  }
}

decl_serde! {
  struct PingResult {
    ping,
    players_total,
    players_game,
  }
}

decl_serde! {
  struct CommandReply {
    ty,
    text => { serialize_text_large, deserialize_text_large }
  }
}

decl_serde! {
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
}

decl_serde! {
  struct PlayerLeave {
    id
  }
}

decl_serde! {
  struct PlayerUpdate {
    clock,
    id,
    keystate,
    upgrades,
    pos => { serialize_pos24, deserialize_pos24 },
    rot => { serialize_rot, deserialize_rot },
    speed => { serialize_vel, deserialize_vel }
  }
}

decl_serde! {
  struct PlayerFireProjectile {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    speed => { serialize_vel, deserialize_vel },
    accel => { serialize_accel, deserialize_accel },
    max_speed => { serialize_speed, deserialize_speed }
  }
}

decl_serde! {
  // Note: Need to implement this one manually
  struct PlayerFire {
    clock,
    id,
    energy => { serialize_energy, deserialize_energy },
    energy_regen => { serialize_regen, deserialize_regen },
    projectiles => { serialize_array_small, deserialize_array_small },
  }
}

decl_serde! {
  struct PlayerRespawn {
    id,
    pos => { serialize_pos24, deserialize_pos24 },
    rot => { serialize_rot, deserialize_rot },
    upgrades
  }
}

decl_serde! {
  struct PlayerFlag {
    id,
    flag
  }
}

decl_serde! {
  struct PlayerLevel {
    id,
    ty,
    level
  }
}

decl_serde! {
  struct PlayerHitPlayer {
    id,
    health => { serialize_energy, deserialize_energy },
    health_regen => { serialize_regen, deserialize_regen }
  }
}

decl_serde! {
  // TODO: Need manual fixup
  struct PlayerHit {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    owner,
    players => { serialize_array_small, deserialize_array_small }
  }
}

decl_serde! {
  // TODO: Needs to be done manually as well
  struct PlayerKill {
    id,
    killer => { serialize_option_player, deserialize_option_player },
    pos => { serialize_pos, deserialize_pos }
  }
}

decl_serde! {
  struct PlayerUpgrade {
    upgrades,
    ty,
    speed,
    defense,
    energy,
    missile
  }
}

decl_serde! {
  struct PlayerType {
    id,
    ty
  }
}

decl_serde! {
  struct PlayerPowerup {
    ty,
    duration
  }
}

decl_serde! {
  struct PlayerReteamPlayer {
    id,
    team
  }
}

decl_serde! {
  struct PlayerReteam {
    players => { serialize_array_large, deserialize_array_large }
  }
}

decl_serde! {
  struct GameFlag {
    ty,
    flag,
    id => { serialize_option_player, deserialize_option_player },
    pos => { serialize_pos24, deserialize_pos24 },
    blueteam,
    redteam
  }
}

decl_serde! {
  struct GameSpectate {
    id
  }
}

decl_serde! {
  struct GamePlayersAlive {
    players
  }
}

decl_serde! {
  struct GameFirewall {
    ty,
    status,
    pos => { serialize_pos, deserialize_pos },
    radius,
    speed
  }
}

decl_serde! {
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
}

decl_serde! {
  struct EventRepelMob {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    speed => { serialize_vel, deserialize_vel },
    accel => { serialize_accel, deserialize_accel },
    max_speed => { serialize_speed, deserialize_speed }
  }
}

decl_serde! {
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
}

decl_serde! {
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
}

decl_serde! {
  struct EventBounce {
    clock,
    id,
    keystate,
    pos => { serialize_pos24, deserialize_pos24 },
    rot => { serialize_rot, deserialize_rot },
    speed => { serialize_vel, deserialize_vel }
  }
}

decl_serde! {
  struct EventStealth {
    id,
    state,
    energy => { serialize_energy, deserialize_energy },
    energy_regen => { serialize_regen, deserialize_regen }
  }
}

decl_serde! {
  struct EventLeaveHorizon {
    ty,
    id
  }
}

decl_serde! {
  struct MobUpdate {
    clock,
    id,
    ty,
    pos => { serialize_pos, deserialize_pos },
    speed => { serialize_vel, deserialize_vel },
    accel => { serialize_accel, deserialize_accel },
    max_speed => { serialize_speed, deserialize_speed }
  }
}

decl_serde! {
  struct MobUpdateStationary {
    id,
    ty,
    pos => { serialize_pos_f32, deserialize_pos_f32 }
  }
}

decl_serde! {
  struct MobDespawn {
    id,
    ty
  }
}

decl_serde! {
  struct MobDespawnCoords {
    id,
    ty,
    pos => { serialize_pos, deserialize_pos }
  }
}

decl_serde! {
  struct ScoreUpdate {
    id,
    score,
    earnings,
    upgrades,
    total_kills,
    total_deaths
  }
}

decl_serde! {
  struct ScoreBoardData {
    id,
    score,
    level
  }
}

decl_serde! {
  struct ScoreBoardRanking {
    id,
    pos => { serialize_low_res_pos, deserialize_low_res_pos }
  }
}

decl_serde! {
  struct ScoreBoard {
    data => { serialize_array_large, deserialize_array_large },
    rankings => { serialize_array_large, deserialize_array_large }
  }
}

decl_serde! {
  struct ScoreDetailedFFAEntry {
    id,
    level,
    score,
    kills,
    deaths,
    damage,
    ping
  }
}

decl_serde! {
  struct ScoreDetailedFFA {
    scores => { serialize_array_large, deserialize_array_large }
  }
}

decl_serde! {
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
}

decl_serde! {
  struct ScoreDetailedCTF {
    scores => { serialize_array_large, deserialize_array_large }
  }
}

decl_serde! {
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
}

decl_serde! {
  struct ScoreDetailedBTR {
    scores => { serialize_array_large, deserialize_array_large }
  }
}

decl_serde! {
  struct ChatTeam {
    id,
    text => { serialize_text_small, deserialize_text_small }
  }
}

decl_serde! {
  struct ChatPublic {
    id,
    text => { serialize_text_small, deserialize_text_small }
  }
}

decl_serde! {
  struct ChatSay {
    id,
    text => { serialize_text_small, deserialize_text_small }
  }
}

decl_serde! {
  struct ChatWhisper {
    from,
    to,
    text => { serialize_text_small, deserialize_text_small }
  }
}

decl_serde! {
  struct ChatVoteMutePassed {
    id
  }
}

decl_serde! {
  struct ServerMessage {
    ty,
    duration,
    text => { serialize_text_large, deserialize_text_large }
  }
}

decl_serde! {
  struct ServerCustom {
    ty,
    data => { serialize_text_large, deserialize_text_large }
  }
}

decl_serde! {
  struct Error {
    error
  }
}

decl_consts! {
  const Login = 0;
  const Login2 = 0;
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

packet_serialize! {
  enum ServerPacket {
    Login(x),
    Login2(x),
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

packet_deserialize! {
  enum ServerPacket {
    // Login(x),
    // Login2(x),
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

  match de {
    Login::V5_PACKET_NO => {
      let login = de.deserialize().with_context("Login")?;

      if de.remainder().is_empty() {
        return Ok(ServerPacket::Login(login));
      }

      Ok(ServerPacket::Login2(Login2 {
        login,
        config: de.deserialize_text_large()
          .with_context("config")
          .with_context("Login2")?,
        bots: de.deserialize_array_large()
          .with_context("bots")
          .with_context("Login2")?,
      }))
    }
  }
}
