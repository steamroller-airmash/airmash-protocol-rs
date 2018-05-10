//! Messages send from server to client

serde_decls! {
    /* READ BEFORE EDITING THIS FILE:
        Serialization/Deserialization is done in
        the order that the fields are declared.
        Changing the order of the fields without
        being aware of this will break things!
    */


    pub struct LoginPlayer {
        pub id: u16,
        pub status: u8,
        pub level: u8,
        pub name: text,
        pub ty: u8,
        pub team: u16,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub rot: rotation,
        pub flag: u16,
        pub upgrades: u8
    }

    pub struct Login {
        pub success: bool,
        pub id: u16,
        pub team: u16,
        pub clock: u32,
        pub token: text,
        pub ty: u8,
        pub room: text,
        pub players: array[LoginPlayer]
    }

    pub struct Backup {}

    pub struct Ping {
        pub clock: u32,
        pub num: u32
    }

    pub struct PingResult {
        pub ping: u16,
        pub players_total: u32,
        pub players_game: u32
    }

    //pub struct Ack { }

    pub struct CommandReply {
        pub ty: u8,
        pub text: textbig
    }

    pub struct PlayerNew {
        pub id: u16,
        pub status: u8,
        pub name: text,
        pub ty: u8,
        pub team: u16,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub rot: rotation,
        pub flag: u16,
        pub upgrades: u8
    }

    pub struct PlayerLeave {
        pub id: u16
    }

    pub struct PlayerUpdate {
        pub clock: u32,
        pub id: u16,
        pub keystate: u8,
        pub upgrades: u8,
        pub pos_x: coord24,
        pub pos_y: coord24,
        pub rot: rotation,
        pub speed_x: speed,
        pub speed_y: speed
    }

    pub struct PlayerFireProjectile {
        pub id: u16,
        pub ty: u8,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub speed_x: speed,
        pub speed_y: speed,
        pub accel_x: accel,
        pub accel_y: accel,
        pub max_speed: speed
    }

    pub struct PlayerFire {
        pub clock: u32,
        pub id: u16,
        pub energy: healthnergy,
        pub energy_regen: regen,
        pub projectiles: arraysmall[PlayerFireProjectile]
    }

    pub struct PlayerSay {
        pub id: u16,
        pub text: text
    }

    pub struct PlayerRespawn {
        pub id: u16,
        pub pos_x: coord24,
        pub pos_y: coord24,
        pub rot: rotation,
        pub upgrades: u8
    }

    pub struct PlayerFlag {
        pub id: u16,
        pub flag: u16
    }

    pub struct PlayerHitPlayer {
        pub id: u16,
        pub health: healthnergy,
        pub health_regen: regen
    }

    pub struct PlayerHit {
        pub id: u16,
        pub ty: u8,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub owner: u16,
        pub players: arraysmall[PlayerHitPlayer]
    }

    pub struct PlayerKill {
        pub id: u16,
        pub killer: u16,
        pub pos_x: coordx,
        pub pos_y: coordy
    }

    pub struct PlayerUpgrade {
        pub upgrades: u16,
        pub ty: u8,
        pub speed: u8,
        pub defense: u8,
        pub energy: u8,
        pub missile: u8
    }

    pub struct PlayerType {
        pub id: u16,
        pub ty: u8
    }

    pub struct PlayerPowerup {
        pub ty: u8,
        pub duration: u32
    }

    pub struct PlayerLevel {
        pub id: u16,
        pub ty: u8,
        pub level: u8
    }

    pub struct PlayerReteamPlayer {
        pub id: u16,
        pub team: u16
    }

    pub struct PlayerReteam {
        pub players: array[PlayerReteamPlayer]
    }

    pub struct GameFlag {
        pub ty: u8,
        pub flag: u8,
        pub id: u16,
        pub pos_x: coord24,
        pub pos_y: coord24,
        pub blueteam: u8,
        pub redteam: u8
    }

    pub struct GameSpectate {
        pub id: u16
    }

    pub struct GamePlayersAlive {
        pub players: u16
    }

    pub struct GameFireWall {
        pub ty: u8,
        pub status: u8,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub radius: f32,
        pub speed: f32
    }

    pub struct EventRepelPlayer {
        pub id: u16,
        pub keystats: u8,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub rot: rotation,
        pub speed_x: speed,
        pub speed_y: speed,
        pub energy: healthnergy,
        pub energy_regen: regen,
        pub player_health: healthnergy,
        pub player_health_regen: regen
    }

    pub struct EventRepelMobs {
        pub id: u16,
        pub ty: u8,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub speed_x: speed,
        pub speed_y: speed,
        pub accel_x: accel,
        pub accel_y: accel,
        pub max_speed: speed
    }

    pub struct EventRepel {
        pub clock: u32,
        pub id: u16,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub rot: rotation,
        pub speed_x: speed,
        pub speed_y: speed,
        pub energy: healthnergy,
        pub energy_regen: regen,
        pub players: arraysmall[EventRepelPlayer],
        pub mobs: arraysmall[EventRepelMobs]
    }

    pub struct EventBoost {
        pub clock: u32,
        pub id: u16,
        pub boost: bool,
        pub pos_x: coord24,
        pub pos_y: coord24,
        pub rot: rotation,
        pub speed_x: speed,
        pub speed_y: speed,
        pub energy: healthnergy,
        pub energy_regen: regen
    }

    pub struct EventBounce {
        pub clock: u32,
        pub id: u16,
        pub keystate: u8,
        pub pos_x: coord24,
        pub pos_y: coord24,
        pub rot: rotation,
        pub speed_x: speed,
        pub speed_y: speed
    }

    pub struct EventStealth {
        pub id: u16,
        pub state: bool,
        pub energy: healthnergy,
        pub energy_regen: regen
    }

    pub struct EventLeaveHorizon {
        pub ty: u8,
        pub id: u16
    }

    pub struct MobUpdate {
        pub clock: u32,
        pub id: u16,
        pub ty: u8,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub speed_x: speed,
        pub speed_y: speed,
        pub accel_x: accel,
        pub accel_y: accel,
        pub max_speed: speed
    }

    pub struct MobUpdateStationary {
        pub id: u16,
        pub ty: u8,
        pub pos_x: f32,
        pub pos_y: f32
    }

    pub struct MobDespawnCoords {
        pub id: u16,
        pub ty: u8,
        pub pos_x: coordx,
        pub pos_y: coordy
    }

    pub struct ScoreUpdate {
        pub id: u16,
        pub score: u32,
        pub earnings: u32,
        pub upgrades: u16,
        pub total_kills: u32,
        pub total_deaths: u32
    }

    pub struct ScoreBoardData {
        pub id: u16,
        pub score: u32,
        pub level: u8
    }

    pub struct ScoreBoardRankings {
        pub id: u16,
        pub x: u8,
        pub y: u8
    }

    pub struct ScoreBoard {
        pub data: array[ScoreBoardData],
        pub rankings: array[ScoreBoardRankings]
    }

    pub struct ScoreDetailedFFAScore {
        pub id: u16,
        pub level: u8,
        pub score: u32,
        pub kills: u16,
        pub deaths: u16,
        pub damage: f32,
        pub ping: u16
    }

    pub struct ScoreDetailedFFA {
        pub scores: array[ScoreDetailedFFAScore]
    }

    pub struct ScoreDetailedCTFScore {
        pub id: u16,
        pub level: u8,
        pub captures: u16,
        pub score: u32,
        pub kills: u16,
        pub deaths: u16,
        pub damage: f32,
        pub ping: u16
    }

    pub struct ScoreDetailedCTF {
        pub scores: array[ScoreDetailedCTFScore]
    }

    pub struct ScoreDetailedBTRScore {
        pub id: u16,
        pub level: u8,
        pub alive: bool,
        pub wins: u16,
        pub score: u32,
        pub kills: u16,
        pub deaths: u16,
        pub damage: f32,
        pub ping: u16
    }

    pub struct ScoreDetailedBTR {
        pub scores: array[ScoreDetailedBTRScore]
    }

    pub struct ChatTeam {
        pub id: u16,
        pub text: text
    }

    pub struct ChatPublic {
        pub id: u16,
        pub text: text
    }

    pub struct ChatSay {
        pub id: u16,
        pub text: text
    }

    pub struct ChatWhisper {
        pub from: u16,
        pub to: u16,
        pub text: text
    }

    pub struct ChatVoteMutePassed {
        pub id: u16
    }

    //pub struct ChatVoteMuted { }

    pub struct ServerMessage {
        pub ty: u8,
        pub duration: u32,
        pub text: textbig
    }

    pub struct ServerCustom {
        pub ty: u8,
        pub data: textbig
    }
}

pub enum ServerPacket {
    Login(Login),
    Backup,
    Ping(Ping),
    PingResult(PingResult),
    Ack,
    CommandReply(CommandReply),
    PlayerNew(PlayerNew),
    PlayerLeave(PlayerLeave),
    PlayerUpdate(PlayerUpdate),
    PlayerFire(PlayerFire),
    PlayerSay(PlayerSay),
    PlayerRespawn(PlayerRespawn),
    PlayerFlag(PlayerFlag),
    PlayerHit(PlayerHit),
    PlayerKill(PlayerKill),
    PlayerType(PlayerType),
    PlayerLevel(PlayerLevel),
    PlayerReteam(PlayerReteam),
    GameFlag(GameFlag),
    GameSpectate(GameSpectate),
    GamePlayersAlive(GamePlayersAlive),
    GameFireWall(GameFireWall),
    EventRepel(EventRepel),
    EventBoost(EventBoost),
    EventBounce(EventBounce),
    EventStealth(EventStealth),
    EventLeaveHorizon(EventLeaveHorizon),
    MobUpdate(MobUpdate),
    MobUpdateStationary(MobUpdateStationary),
    ScoreUpdate(ScoreUpdate),
    ScoreBoard(ScoreBoard),
    ScoreDetailedFFA(ScoreDetailedFFA),
    ScoreDetailedCTF(ScoreDetailedCTF),
    ScoreDetailedBTR(ScoreDetailedBTR),
    ChatTeam(ChatTeam),
    ChatPublic(ChatPublic),
    ChatSay(ChatSay),
    ChatWhisper(ChatWhisper),
    ChatVoteMutePassed(ChatVoteMutePassed),
    ChatVoteMuted,
    ServerMessage(ServerMessage),
    ServerCustom(ServerCustom),
}
