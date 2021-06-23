
macro_rules! impl_from_newtype_inner {
  ($enum:tt, $type:tt) => {
    impl From<$type> for $enum {
      fn from(v: $type) -> Self {
        $enum::$type(v)
      }
    }
  };
}

macro_rules! impl_from_empty_inner {
  ($enum:tt, $type:tt) => {
    impl From<$type> for $enum {
      fn from(_: $type) -> Self {
        $enum::$type
      }
    }
  };
}
