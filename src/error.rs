use std;
use std::str::Utf8Error;

#[derive(Debug, Clone)]
pub enum Error {
    Eof,
    TrailingBytes,
    Utf8Error(Utf8Error),
    InvalidPacketType,
    InvalidKeyCode(u8),
    ArrayLengthTooBig,
    InvalidPlaneType(u8),
    InvalidFlag(u16)
}

impl Error {
    pub(self) fn desc(&self) -> &str {
        match self {
            &Error::Eof => "Unexpected end of message reached.",
            &Error::TrailingBytes => "Unexpected remaining bytes.",
            &Error::Utf8Error(_) => "Invalid utf-8 in string.",
            &Error::InvalidPacketType => "Invalid packet type.",
            &Error::InvalidKeyCode(_) => "Invalid key code.",
            &Error::ArrayLengthTooBig => "Array too large to be serialized, maybe textbig or array types should be used.",
            &Error::InvalidPlaneType(_) => "Invalid plane type",
            &Error::InvalidFlag(_) => "Invalid flag code"
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.desc())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.desc()
    }
}
