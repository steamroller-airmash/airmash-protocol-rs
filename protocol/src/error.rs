use std;

#[derive(Debug, Clone)]
pub enum Error {
	Eof,
	TrailingBytes
}

impl Error {
	pub(self) fn desc(&self) -> &str {
		match self {
			&Error::Eof => "unexpected end of message reached",
			&Error::TrailingBytes => "unexpected remaining bytes"
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
