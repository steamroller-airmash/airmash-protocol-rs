#[macro_use]
mod macros;
mod flag_code;

pub use self::flag_code::FlagCode;

decl_enum! {
  /// Specifies whether the debug reply to a command should
  /// open a popup or be displayed in the chat window.
  #[non_exhaustive]
  ##[default = ShowInPopup]
  pub enum CommandReplyType {
    ShowInConsole = 0,
    /// Technically this should be any value other than `0`,
    /// the [`From`][0] integer implementation for this enum deals
    /// with that.
    ///
    /// [0]: https://doc.rust-lang.org/std/convert/trait.From.html
    ShowInPopup = 1,
  }

  /// Details on how the mob despawned. (i.e. whether
  /// it's lifetime ended or it collided with some
  /// other object)
  #[non_exhaustive]
  pub enum DespawnType {
    LifetimeEnded = 0,
    Collided = 1,
  }

  /// All error codes that can be sent to the client.
  ///
  /// These are all server errors that the vanilla AIRMASH
  /// client (and the current STARMASH client) understands.
  #[non_exhaustive]
  ##[default = UnknownError]
  pub enum ErrorType {
    DisconnectedForPacketFlooding = 1,
    BannedForPacketFlooding = 2,
    Banned = 3,
    IdleRequiredBeforeRespawn = 5,
    AfkTimeout = 6,
    Kicked = 7,
    InvalidLogin = 8,
    IncorrectProtocolLevel = 9,
    AccountBanned = 10,
    AccountAlreadyLoggedIn = 11,
    NoRespawnInBTR = 12,
    IdleRequiredBeforeSpectate = 13,
    NotEnoughUpgrades = 20,
    ChatThrottled = 30,
    FlagChangeThrottled = 31,
    UnknownCommand = 100,

    UnknownError = 255,
  }

  /// TODO: Reverse engineer
  #[non_exhaustive]
  pub enum FirewallStatus {}

  /// TODO: Reverse engineer
  #[non_exhaustive]
  pub enum FirewallUpdateType {}

  /// Flag update type
  ///
  /// Used to indicate whether the flag is now being
  /// carried by a player or whether the update
  /// sets the position of the flag directly.
  ///
  /// Used in:
  /// - TODO
  ///
  /// Implementors Note: This had a `TODO: rev-eng`
  /// comment on it but it doesn't seem to be missing
  /// any values.
  #[non_exhaustive]
  pub enum FlagUpdateType {
    Position = 1,
    Carrier = 2,
  }

  /// Game Type.
  ///
  /// Hopefully self explanatory, used to indicate to
  /// the client which game is being played. The client
  /// uses this to decide on player colouring and
  /// whether or not to show the flags in-game.
  /// It will also correspond with the type of detailed
  /// score ([`ScoreDetailedFFA`][0], [`ScoreDetailedCTF`][1],
  /// or [`ScoreDetailedBTR`][2]) that the client expects
  /// to receive.
  ///
  /// Used in:
  /// - TODO
  ///
  /// [0]: server/struct.ScoreDetailedFFA.html
  /// [1]: server/struct.ScoreDetailedCTF.html
  /// [2]: server/struct.ScoreDetailedBTR.html
  #[non_exhaustive]
  ##[default = FFA]
  pub enum GameType {
    FFA = 1,
    CTF = 2,
    BTR = 3,
  }

  /// The key that's had it's state changed.
  /// This is only used for client -> server
  /// communication.
  ///
  /// It is used in the following packets:
  /// - TODO
  #[non_exhaustive]
  pub enum KeyCode {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
    Fire = 5,
    Special = 6,
  }

  /// Indicates the type of entity that just
  /// went outside of the player's horizon.
  ///
  /// TODO: Complete reverse engineering this.
  /// NOTE: The values here aren't in any way
  /// certain and should be verified before
  /// relying upon them.
  #[non_exhaustive]
  pub enum LeaveHorizonType {
    Player = 0,
    Mob = 1,
  }

  /// Types of all mobs present in the game.
  ///
  /// In AIRMASH, mobs are any non-player and non-wall
  /// items that can be interacted with. This includes
  /// powerups, upgrades, and all missiles.
  ///
  /// Used by:
  /// - TODO
  #[non_exhaustive]
  pub enum MobType {
    PredatorMissile = 1,
    GoliathMissile = 2,
    MohawkMissile = 3,
    Upgrade = 4,
    TornadoSingleMissile = 5,
    TornadoTripleMissile = 6,
    ProwlerMissile = 7,
    Shield = 8,
    Inferno = 9,
  }

  /// Used to indicate the type of plane
  /// that the packet refers to.
  ///
  /// Used in:
  /// - TODO
  #[non_exhaustive]
  ##[default = Predator]
  pub enum PlaneType {
    Predator = 1,
    Goliath = 2,
    Mohawk = 3,
    Tornado = 4,
    Prowler = 5,
  }

  /// Indicate whether a player levelled up, or has
  /// just logged in and their level is being communicated
  /// to the client.
  #[non_exhaustive]
  ##[default = Login]
  pub enum PlayerLevelType {
    Login = 0,
    LevelUp = 1,
  }

  /// Flag for indicating whether a player is
  /// alive or dead.
  ///
  /// This is used in the following packets:
  /// - [`Login`][0] (specifically [`LoginPlayer`][1])
  /// - [`PlayerNew`][2]
  ///
  /// [0]: server/struct.login.html
  /// [1]: server/struct.loginplayer.html
  /// [2]: server/struct.playernew.html
  #[non_exhaustive]
  ##[default = Alive]
  pub enum PlayerStatus {
    Alive = 0,
    Dead = 1,
  }

  /// TODO: Reverse engineer
  #[non_exhaustive]
  ##[default = Shield]
  pub enum PowerupType {
    Shield = 1,
    /// This is just a guess.
    /// TODO: Verify
    Inferno = 2,
  }

  /// Specific identifiers for server custom messages.
  ///
  /// TODO: Reverse Engineer
  #[non_exhaustive]
  pub enum ServerCustomType {
    /// TODO: Determine if this name is accurate
    BTRWin = 1,
    /// TODO: Determine if this name is accurate
    CTFWin = 2,
  }

  /// Type specifier for server banner messages.
  ///
  /// TODO: Reverse engineer
  #[non_exhaustive]
  pub enum ServerMessageType {
    TimeToGameStart = 1,
    /// TODO: Verify the value of this one
    Flag = 2,
    /// New Type, used by this server for shutdown message
    /// (once they work)
    Shutdown = 15,
    /// New Type, used by this server for banner messages
    /// on player join.
    Banner = 16,
  }

  /// All upgrade types.
  #[non_exhaustive]
  ##[default = None]
  pub enum UpgradeType {
    /// This seems to be sent by the official server when a
    /// player leaves. Packets with this value are ignored
    /// by the client, so they don't seem to affect gameplay
    /// at all.
    None = 0,
    Speed = 1,
    Defense = 2,
    Energy = 3,
    Missile = 4,
  }
}

impl MobType {
  pub fn is_missile(&self) -> bool {
    use self::MobType::*;

    matches!(
      self,
      PredatorMissile
        | GoliathMissile
        | MohawkMissile
        | TornadoSingleMissile
        | TornadoTripleMissile
        | ProwlerMissile
    )
  }

  pub fn is_powerup(&self) -> bool {
    use self::MobType::*;

    matches!(self, Shield | Inferno)
  }
}
