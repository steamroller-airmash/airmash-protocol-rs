#[macro_use]
mod macros;
mod flag_code;
#[cfg(test)]
mod tests;

pub use self::flag_code::FlagCode;

decl_enum! {
  /// Specifies whether the debug reply to a command should
  /// open a popup or be displayed in the chat window.
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
  pub enum DespawnType {
    LifetimeEnded = 0,
    Collided = 1,
  }

  /// All error codes that can be sent to the client.
  ///
  /// These are all server errors that the vanilla AIRMASH
  /// client (and the current STARMASH client) understands.
  pub enum ErrorType {
    PacketFloodingDisconnect = 1,
    PacketFloodingBan = 2,
    Banned = 3,
    /// This error doesn't actually show an error on the client side but
    /// instead forces the client to reload the page.
    ForceClientReload = 4,
    IdleRequiredBeforeRespawn = 5,
    AfkTimeout = 6,
    Kicked = 7,
    InvalidLogin = 8,
    IncorrectProtocol = 9,
    AccountBanned = 10,
    AccountAlreadyLoggedIn = 11,
    NoRespawnInBTR = 12,
    IdleRequiredBeforeSpectate = 13,
    NotEnoughUpgrades = 20,
    ChatThrottled = 30,
    FlagChangeThrottled = 31,
    UnknownCommand = 100,
  }

  /// This is used to control whether the firewall exists in BTR.
  ##[catchall = Present]
  pub enum FirewallStatus {
    /// If this status is sent then the client will remove the ring of fire.
    Removed = 0,
    /// Otherwise, any other status means it should exist.
    Present = 1,
  }

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
  ##[default = FFA]
  pub enum GameType {
    FFA = 1,
    CTF = 2,
    BTR = 3,
  }

  /// The key that's had it's state changed. 
  /// 
  /// This is only used for client to server communication. For communications
  /// back from server to client see [`ServerKeyState`].
  ///
  /// It is used in the following packets:
  /// - TODO
  /// 
  /// [`ServerKeyState`]: crate::ServerKeyState
  pub enum KeyCode {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
    Fire = 5,
    Special = 6,
  }

  /// Indicates the type of entity that just left the player's horizon.
  ##[catchall = Mob]
  pub enum LeaveHorizonType {
    Player = 0,
    Mob = 1,
  }

  /// Types of all mobs present in the game.
  ///
  /// In AIRMASH, mobs are any non-player and non-wall items that can be
  /// interacted with. This includes powerups, upgrades, and all missiles.
  ///
  /// Used by:
  /// - TODO
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

  /// Used to indicate the type of plane that the packet refers to.
  ///
  /// Used in:
  /// - TODO
  ##[default = Predator]
  pub enum PlaneType {
    Predator = 1,
    Goliath = 2,
    Mohawk = 3,
    Tornado = 4,
    Prowler = 5,
  }

  /// Indicate whether a player levelled up, or has just logged in and their
  /// level is being communicated to the client.
  ##[default = Login]
  pub enum PlayerLevelType {
    Login = 0,
    LevelUp = 1,
  }

  /// Flag for indicating whether a player is alive or dead.
  ///
  /// This is used in the following packets:
  /// - [`Login`][0] (specifically [`LoginPlayer`][1])
  /// - [`PlayerNew`][2]
  ///
  /// [0]: server/struct.login.html
  /// [1]: server/struct.loginplayer.html
  /// [2]: server/struct.playernew.html
  ##[default = Alive]
  pub enum PlayerStatus {
    Alive = 0,
    Dead = 1,
  }

  /// The type of powerup effect that a player has.
  ##[default = Shield]
  pub enum PowerupType {
    Shield = 1,
    Inferno = 2,
  }

  /// Specific identifiers for server custom messages.
  pub enum ServerCustomType {
    /// Triggers the game-end screen in BTR.
    BTR = 1,
    /// Triggers the game-end screen in CTF.
    CTF = 2,

    /// For suggesting a different game server for the player to switch to.
    SwitchGameSuggestion = 100,
  }

  /// Type specifier for server banner messages.
  ///
  /// This _mostly_ doesn't correspond to behaviour within the default client.
  /// The one exception is that [`Informational`] messages will keep showing
  /// even if other messages are shown. However, alternate clients may use the
  /// custom classes to show different messages in different ways.
  ///
  /// [`Informational`]: ServerMessageType::Informational
  pub enum ServerMessageType {
    /// Used by the CTF server to show messages counting down to the start of
    /// the next game.
    TimeToGameStart = 1,
    /// An informational message related to the current game state. This won't
    /// be overwritten by any other message category so it should be used for
    /// important game-related information.
    ///
    /// # Usage Examples
    /// - CTF uses this server message type to show flag-related updates.
    Informational = 2,
  }

  /// All upgrade types.
  ##[default = None]
  pub enum UpgradeType {
    /// This seems to be sent by the official server when a player leaves.
    /// Packets with this value are ignored by the client, so they don't seem
    /// to affect gameplay at all.
    None = 0,
    Speed = 1,
    Defense = 2,
    Energy = 3,
    Missile = 4,
  }
}

#[allow(non_upper_case_globals)]
impl ServerCustomType {
  // Old aliases for CTF and BTR - kept here for backcompat
  #[deprecated]
  pub const BTRWin: Self = Self::BTR;
  #[deprecated]
  pub const CTFWin: Self = Self::CTF;
}

#[allow(non_upper_case_globals)]
impl ErrorType {
  #[deprecated(
    since = "0.6.0",
    note = "use ErrorType::PacketFloodingDisconnect instead"
  )]
  pub const DisconnectedForPacketFlooding: Self = Self::PacketFloodingDisconnect;
  #[deprecated(since = "0.6.0", note = "use ErrorType::PacketFloodingBan instead")]
  pub const BannedForPacketFlooding: Self = Self::PacketFloodingBan;
  #[deprecated(since = "0.6.0", note = "use ErrorType::IncorrectProtocol instead")]
  pub const IncorrectProtocolLevel: Self = Self::IncorrectProtocol;
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

#[allow(non_upper_case_globals)]
impl ServerMessageType {
  pub const Flag: Self = Self::Informational;
  pub const Shutdown: Self = Self::Unknown(15);

  /// Unofficial message type. Used by the rust server for banner messages at
  /// login.
  pub const Banner: Self = Self::Unknown(16);
}
