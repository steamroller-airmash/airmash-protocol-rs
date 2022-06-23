//! The debug format for Vector2 is rather large. This module adds a custom
//! debug implementation for packets which use it that is more compact.

use std::fmt::{Debug, Formatter, Result};

use crate::types::Vector2;

pub(crate) fn fmt_vector(v: &Vector2, f: &mut Formatter) -> Result {
  write!(f, "({}, {})", v.x, v.y)
}

pub(crate) fn fmt_opt_vector(v: &Option<Vector2>, f: &mut Formatter) -> Result {
  v.map(|v| (v.x, v.y)).fmt(f)
}
