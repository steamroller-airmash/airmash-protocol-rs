//! Messages sent from server to client

serde_decls! {
    /* READ BEFORE EDITING THIS FILE:
        Serialization/Deserialization is done in
        the order that the fields are declared.
        Changing the order of the fields without
        being aware of this will break things!
    */

    /// Initial data passed in for a 
    /// player when the server starts.
    /// 
    /// This is an element of the `players`
    /// array within the 
    /// [`Login`](struct.login.html)
    /// packet.
    #[derive(Default, Clone, Debug)]
    pub struct LoginPlayer {
        /// The id of the player.
        pub id: u16,
        pub status: u8,
        /// The level of the player.
        pub level: u8,
        /// The player's name. This may
        /// be different than the name
        /// requested in the 
        /// [`Login`](../client/struct.login.html)
        /// packet.
        pub name: text,
        /// The type of plane the player is flying.
        pub ty: u8,
        /// The current team that the player is on.
        pub team: u16,
        /// The X position of the player.
        pub pos_x: coordx,
        /// The Y positoin of the player.
        pub pos_y: coordy,
        /// The player's current rotation.
        pub rot: rotation,
        /// The current flag of the player.
        pub flag: u16,
        pub upgrades: u8
    }

    /// Initial login packet sent to the server.
    #[derive(Default, Clone, Debug)]
    pub struct Login {
        /// Whether the login was successful.
        pub success: bool,
        /// The id of the just-logged-in player.
        pub id: u16,
        /// The team of the just-logged-in player.
        pub team: u16,
        /// Current server clock
        pub clock: u32,
        /// The login token used by the current 
        /// player, or `"none"`.
        pub token: text,
        /// The plane that the current player 
        /// is flying.
        pub ty: u8,
        /// The room that the current player
        /// has just logged into.
        pub room: text,
        /// Data on all players within the 
        /// current room.
        pub players: array[LoginPlayer]
    }

    //#[derive(Default, Clone, Copy, Debug)]
    //pub struct Backup {}

    /// A ping request by the server. 
    /// 
    /// All clients must respond with a
    /// [`Pong`](../client/struct.pong.html)
    /// with `num` set to the same value
    /// as this packet. If a client does
    /// not do this, the client will be
    /// disconnected by the server.
    #[derive(Default, Clone, Copy, Debug)]
    pub struct Ping {
        /// Current server clock.
        pub clock: u32,
        /// Packet number
        pub num: u32
    }

    /// Resulting ping data sent back
    /// from the server.
    #[derive(Default, Clone, Copy, Debug)]
    pub struct PingResult {
        /// Ping of the current player.
        pub ping: u16,
        /// Total number of players online.
        pub players_total: u32,
        /// Number of players in the current game.
        pub players_game: u32
    }

    //pub struct Ack { }

    /// Reply to a
    /// [`Command`](client/struct.command.html).
    #[derive(Default, Clone, Debug)]
    pub struct CommandReply {
        pub ty: u8,
        pub text: textbig
    }

    /// Data for a newly-arrived player.
    #[derive(Default, Clone, Debug)]
    pub struct PlayerNew {
        /// The id of the new player.
        pub id: u16,
        pub status: u8,
        /// The name of the new player.
        pub name: text,
        /// Plane of the new player.
        pub ty: u8,
        /// Team of the new player.
        pub team: u16,
        /// X position of the new player.
        pub pos_x: coordx,
        /// Y position of the new player.
        pub pos_y: coordy,
        /// Rotation of the new player.
        pub rot: rotation,
        /// Flag of the newly-arrived player.
        pub flag: u16,
        pub upgrades: u8
    }

    /// Packet sent when a player leaves the room.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerLeave {
        pub id: u16
    }

    /// Movement update for a player.
    #[derive(Default, Clone, Debug)]
    pub struct PlayerUpdate {
        /// Server clock
        pub clock: u32,
        /// Player id
        pub id: u16,
        /// Keys pressed by a player
        pub keystate: u8,
        pub upgrades: u8,
        /// X position of player
        pub pos_x: coord24,
        /// Y position of player
        pub pos_y: coord24,
        /// Rotation of player
        pub rot: rotation,
        /// Speed in X direction
        pub speed_x: speed,
        /// Speed in Y direction
        pub speed_y: speed
    }

    /// Data on a projectile fired by a plane.
    /// 
    /// This is data for the `projectiles`
    /// array of a 
    /// [`PlayerFire`](struct.playerfire.html)
    /// packet.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerFireProjectile {
        /// Projectile id
        pub id: u16,
        /// Projectile type
        pub ty: u8,
        /// Current projectile X position
        pub pos_x: coordx,
        /// Current projectile Y position
        pub pos_y: coordy,
        /// Current projectile speed in X direction
        pub speed_x: speed,
        /// Current projectile speed in Y direction
        pub speed_y: speed,
        /// Current projectile acceleration in X direction
        pub accel_x: accel,
        /// Current projectile acceleration in Y direction
        pub accel_y: accel,
        /// The maximum speed that the projectile
        /// can attain
        pub max_speed: speed
    }

    /// Packet indicating that a player has fired projectiles.
    #[derive(Default, Clone, Debug)]
    pub struct PlayerFire {
        /// Current server clock
        pub clock: u32,
        /// Id of firing player
        pub id: u16,
        /// Energy of firing player
        pub energy: healthnergy,
        /// Energy regen of firing player
        pub energy_regen: regen,
        /// All projectiles fired by the player
        pub projectiles: arraysmall[PlayerFireProjectile]
    }

    // TODO: Remove this?
    /// Note: exists within the game code,
    /// but doesn't have a corresponding 
    /// message code. This appears to be 
    /// left over from an earlier version
    /// of the airmash codebase, left here
    /// in case it turns out to actually be
    /// used. However, it appears to be 
    /// identical to ChatSay.
    #[doc(hidden)]
    #[derive(Default, Clone, Debug)]
    pub struct PlayerSay {
        pub id: u16,
        pub text: text
    }

    /// Event fired when a player respawns
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerRespawn {
        /// Id of respawning player
        pub id: u16,
        /// Player X position
        pub pos_x: coord24,
        /// Player Y position
        pub pos_y: coord24,
        /// Player rotation
        pub rot: rotation,
        pub upgrades: u8
    }

    /// Event indicating the a player has 
    /// changed their flag.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerFlag {
        /// Id of player
        pub id: u16,
        /// Id of new flag
        pub flag: u16
    }

    /// Data on a player that has been
    /// hit by a shot fired by another 
    /// player.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerHitPlayer {
        /// Id of player
        pub id: u16,
        /// Health of player
        pub health: healthnergy,
        /// Health regen rate of player.
        pub health_regen: regen
    }

    /// Event for when players have been
    /// hit by a projectile.
    #[derive(Default, Clone, Debug)]
    pub struct PlayerHit {
        /// Projectile id
        pub id: u16,
        /// Projectile type
        pub ty: u8,
        /// X position of projectile
        pub pos_x: coordx,
        /// Y position of projectile
        pub pos_y: coordy,
        /// Projectile owner
        pub owner: u16,
        /// Players hit by projectile
        pub players: arraysmall[PlayerHitPlayer]
    }

    /// Event for when a player has been killed.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerKill {
        /// Id of killed player
        pub id: u16,
        /// Id of the player that killed them
        pub killer: u16,
        /// X position of killed player
        pub pos_x: coordx,
        /// Y position of killed player
        pub pos_y: coordy
    }

    /// Event fired when a player upgrades 
    /// themself.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerUpgrade {
        pub upgrades: u16,
        pub ty: u8,
        pub speed: u8,
        pub defense: u8,
        pub energy: u8,
        pub missile: u8
    }

    /// Event fired when a player changes
    /// their plane.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerType {
        /// Player id
        pub id: u16,
        /// New plane type
        pub ty: u8
    }

    /// Event fired when a player picks
    /// up a powerup.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerPowerup {
        /// The type of the powerup
        pub ty: u8,
        /// The duration of the powerup
        pub duration: u32
    }

    /// Event fired when a player's level
    /// changes.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerLevel {
        /// Id of player
        pub id: u16,
        pub ty: u8,
        /// Player's new level
        pub level: u8
    }

    /// Data an a player that has changed 
    /// teams.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct PlayerReteamPlayer {
        /// Player id
        pub id: u16,
        /// New team of player
        pub team: u16
    }

    /// Event fired when players change teams.
    #[derive(Default, Clone, Debug)]
    pub struct PlayerReteam {
        /// List of players that have changed
        /// teams. Note that this does not 
        /// include players that have remained
        /// on the same team.
        pub players: array[PlayerReteamPlayer]
    }

    /// Flag position update.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct GameFlag {
        pub ty: u8,
        /// Which team's flag is being updated
        pub flag: u8,
        /// Id of carrier, or 0 if flag is no
        /// longer being carried.
        pub id: u16,
        /// Flag X position
        pub pos_x: coord24,
        /// Flag Y position
        pub pos_y: coord24,
        pub blueteam: u8,
        pub redteam: u8
    }

    /// Event indicating which 
    /// player is now being spectated.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct GameSpectate {
        /// Id of player being spectated
        pub id: u16
    }

    /// Packet indicating how many players 
    /// are alive.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct GamePlayersAlive {
        /// Number of players currently alive
        pub players: u16
    }

    /// Update of the "Ring of Fire" in BTR
    #[derive(Default, Clone, Debug, Copy)]
    pub struct GameFireWall {
        pub ty: u8,
        pub status: u8,
        pub pos_x: coordx,
        pub pos_y: coordy,
        pub radius: f32,
        pub speed: f32
    }

    /// Data about players repelled by a goliath
    /// deflect.
    #[derive(Default, Clone, Debug, Copy)]
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

    /// Data about projectiles deflected by a 
    /// goliath repel.
    #[derive(Default, Clone, Debug, Copy)]
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

    /// Event triggered when something
    /// (players or projectiles) is deflected
    /// by a goliath repel.
    #[derive(Default, Clone, Debug)]
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

    /// Event for when a predator begins boosting.
    #[derive(Default, Clone, Debug, Copy)]
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

    /// Event for when a player runs into a wall.
    #[derive(Default, Clone, Debug, Copy)]
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

    /// Event for when a prowler goes into stealth.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct EventStealth {
        pub id: u16,
        pub state: bool,
        pub energy: healthnergy,
        pub energy_regen: regen
    }

    /// Event for when a player goes beyond
    /// the horizon that the server will send
    /// updates for.
    /// 
    /// No more updates will be sent for planes 
    /// outside the horizon once this packet
    /// has been sent.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct EventLeaveHorizon {
        pub ty: u8,
        pub id: u16
    }

    /// Update of missile or powerup
    #[derive(Default, Clone, Debug, Copy)]
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

    /// Update of non-moving mob (powerups)
    #[derive(Default, Clone, Debug, Copy)]
    pub struct MobUpdateStationary {
        pub id: u16,
        pub ty: u8,
        pub pos_x: f32,
        pub pos_y: f32
    }

    /// Event for missile destruction
    #[derive(Default, Clone, Debug, Copy)]
    pub struct MobDespawn {
        pub id: u16,
        pub ty: u8
    }

    /// Event indicating a mob despawned 
    /// at a particular location.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct MobDespawnCoords {
        pub id: u16,
        pub ty: u8,
        pub pos_x: coordx,
        pub pos_y: coordy
    }

    /// Packet indicating stats for the 
    /// current player.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct ScoreUpdate {
        pub id: u16,
        pub score: u32,
        pub earnings: u32,
        pub upgrades: u16,
        pub total_kills: u32,
        pub total_deaths: u32
    }

    /// Leaderboard data, part of the
    /// [`ScoreBoard`](struct.scoreboard.html)
    /// packet.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct ScoreBoardData {
        pub id: u16,
        pub score: u32,
        pub level: u8
    }

    /// Low-res player positions, part of the
    /// [`ScoreBoard`](struct.scoreboard.html)
    /// packet.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct ScoreBoardRankings {
        pub id: u16,
        pub x: u8,
        pub y: u8
    }

    /// Leaderboard + Global player positions
    /// 
    /// This is sent every 5 seconds by the
    /// server and is used by the client to
    /// update the leaderboard and minimap.
    #[derive(Default, Clone, Debug)]
    pub struct ScoreBoard {
        pub data: array[ScoreBoardData],
        pub rankings: array[ScoreBoardRankings]
    }

    /// Per-player data for detailed 
    /// (tab) menu in FFA.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct ScoreDetailedFFAScore {
        pub id: u16,
        pub level: u8,
        pub score: u32,
        pub kills: u16,
        pub deaths: u16,
        pub damage: f32,
        pub ping: u16
    }

    /// Detailed menu (tab) data for FFA.
    #[derive(Default, Clone, Debug)]
    pub struct ScoreDetailedFFA {
        pub scores: array[ScoreDetailedFFAScore]
    }

    /// Per-player data for detailed (tab)
    /// menu in CTF.
    #[derive(Default, Clone, Debug, Copy)]
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

    /// Detailed menu (tab) data for CTF.
    #[derive(Default, Clone, Debug)]
    pub struct ScoreDetailedCTF {
        pub scores: array[ScoreDetailedCTFScore]
    }

    /// Per-player data for detailed (tab) menu
    /// in BTR.
    #[derive(Default, Clone, Debug, Copy)]
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

    /// Detailed menu (tab) data for BTR.
    #[derive(Default, Clone, Debug)]
    pub struct ScoreDetailedBTR {
        pub scores: array[ScoreDetailedBTRScore]
    }

    /// Event for when a team chat has been 
    /// received.
    #[derive(Default, Clone, Debug)]
    pub struct ChatTeam {
        /// Id of chatting player
        pub id: u16,
        /// Message text
        pub text: text
    }

    /// Event for a public chat
    #[derive(Default, Clone, Debug)]
    pub struct ChatPublic {
        /// Id of chatting player
        pub id: u16,
        /// Message text
        pub text: text
    }

    /// Event for a speech bubble by a player
    #[derive(Default, Clone, Debug)]
    pub struct ChatSay {
        /// Id of speaking player
        pub id: u16,
        // Message text
        pub text: text
    }

    /// Event for a whisper message involving
    /// the current player
    #[derive(Default, Clone, Debug)]
    pub struct ChatWhisper {
        /// The player that sent the whisper.
        pub from: u16,
        /// The player that received the whisper.
        pub to: u16,
        /// Message text
        pub text: text
    }

    /// Event indicating that a player has 
    /// been votemuted.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct ChatVoteMutePassed {
        /// Id of votemuted player
        pub id: u16
    }

    //pub struct ChatVoteMuted { }

    /// Banner message sent by the server
    #[derive(Default, Clone, Debug)]
    pub struct ServerMessage {
        pub ty: u8,
        /// Duration that banner message
        /// should remain on the screen.
        pub duration: u32,
        /// HTML text of banner message
        pub text: textbig
    }

    /// End of game packet for CTF and BTR.
    /// 
    /// # CTF
    /// In CTF, the data of this packet contains 
    /// a JSON string with 3 fields.
    /// 
    /// - `w`: The id of the winning team.
    /// - `b`: The bounty given to each player
    /// of the winning team.
    /// - `t`: The time (in seconds) that the
    /// banner should remain on screen before 
    /// closing (unless closed by the player).
    /// 
    /// # BTR
    /// TODO
    #[derive(Default, Clone, Debug)]
    pub struct ServerCustom {
        pub ty: u8,
        pub data: textbig
    }

    /// The client has done an invalid operation or
    /// has been ratelimited or banned.
    #[derive(Default, Clone, Debug, Copy)]
    pub struct Error {
        /// Error code indicating which error
        /// it is.
        pub error: u8
    }
}

/// All possible server packets.
/// 
/// This is an enum of all possible packet 
/// message types. It can be serialized 
/// and deserialized from byte buffers
/// using [`from_bytes`](fn.from_bytes.html)
/// and [`to_bytes`](fn.to_bytes.html).
/// 
/// Some packets do not contain any data
/// and thus do not have any data within
/// their enum variants.
#[derive(Clone, Debug)]
pub enum ServerPacket {
    Login(Login),
    Backup,
    Ping(Ping),
    PingResult(PingResult),
    Ack,
    Error(Error),
    CommandReply(CommandReply),
    PlayerNew(PlayerNew),
    PlayerLeave(PlayerLeave),
    PlayerUpdate(PlayerUpdate),
    PlayerFire(PlayerFire),
    PlayerRespawn(PlayerRespawn),
    PlayerFlag(PlayerFlag),
    PlayerHit(PlayerHit),
    PlayerKill(PlayerKill),
    PlayerUpgrade(PlayerUpgrade),
    PlayerType(PlayerType),
    PlayerPowerup(PlayerPowerup),
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
    MobDespawn(MobDespawn),
    MobDespawnCoords(MobDespawnCoords),
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
