use std::fmt::{self, Display};

pub trait ErrorExt {
  fn with_context(self, field: &'static str) -> Self;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ErrorKind {
  EndOfBuffer,
  InvalidEnumValue,
  ArraySizeTooLarge,
}

#[derive(Clone, Debug)]
pub struct Error {
  kind: ErrorKind,
  context: Vec<&'static str>,
}

impl Error {
  pub fn new(kind: ErrorKind) -> Self {
    Self {
      kind,
      context: Vec::new(),
    }
  }

  pub fn kind(&self) -> ErrorKind {
    self.kind
  }

  pub fn context(&self) -> &[&'static str] {
    &self.context[..]
  }

  fn description(&self) -> &str {
    match self.kind() {
      ErrorKind::EndOfBuffer => "reached end of buffer",
      ErrorKind::InvalidEnumValue => "invalid enum value",
      ErrorKind::ArraySizeTooLarge => "array size too large for type",
    }
  }
}

impl std::error::Error for Error {
  fn description(&self) -> &str {
    Self::description(self)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "failed to parse: {}", self.description())?;
    writeln!(f, "Field stack:")?;

    for field in self.context().iter().copied().rev() {
      writeln!(f, "  - {}", field)?;
    }

    Ok(())
  }
}

impl ErrorExt for Error {
  fn with_context(mut self, field: &'static str) -> Self {
    self.context.push(field);
    self
  }
}

impl<T> ErrorExt for Result<T, Error> {
  fn with_context(self, field: &'static str) -> Self {
    self.map_err(|e| e.with_context(field))
  }
}
