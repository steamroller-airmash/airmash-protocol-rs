macro_rules! decl_enum_from_to {
  {
    $( default = $default:ident; )?
    enum $name:ident;
  } => {
    decl_enum_from_to!($(default = $default, )? $name, u8,   to_u8,   from_u8);
    decl_enum_from_to!($(default = $default, )? $name, u16,  to_u16,  from_u16);
    decl_enum_from_to!($(default = $default, )? $name, u32,  to_u32,  from_u32);
    decl_enum_from_to!($(default = $default, )? $name, u64,  to_u64,  from_u64);
    decl_enum_from_to!($(default = $default, )? $name, u128, to_u128, from_u128);

    decl_enum_from_to!($(default = $default, )? $name, i8,   to_i8,   from_i8);
    decl_enum_from_to!($(default = $default, )? $name, i16,  to_i16,  from_i16);
    decl_enum_from_to!($(default = $default, )? $name, i32,  to_i32,  from_i32);
    decl_enum_from_to!($(default = $default, )? $name, i64,  to_i64,  from_i64);
    decl_enum_from_to!($(default = $default, )? $name, i128, to_i128, from_i128);
  };

  (default = $default:ident, $name:ident, $ty:ident, $to:ident, $from:ident) => {
    impl ::std::convert::From<$ty> for $name {
      fn from(v: $ty) -> Self {
        use ::num_traits::FromPrimitive;
        Self::$from(v).unwrap_or(Self::default())
      }
    }

    impl ::std::convert::From<$name> for $ty {
      fn from(v: $name) -> Self {
        use ::num_traits::ToPrimitive;
        v.$to().expect("Type too large for primitive")
      }
    }
  };
  ($name:ident, $ty:ident, $to:ident, $from:ident) => {
    impl ::std::convert::TryFrom<$ty> for $name {
      type Error = crate::EnumValueOutOfRangeError<$ty>;

      fn try_from(v: $ty) -> Result<Self, Self::Error> {
        use ::num_traits::FromPrimitive;
        match Self::$from(v) {
          Some(x) => Ok(x),
          None => Err(crate::EnumValueOutOfRangeError(v))
        }
      }
    }

    impl ::std::convert::From<$name> for $ty {
      fn from(v: $name) -> Self {
        use ::num_traits::ToPrimitive;
        v.$to().expect("Type too large for primitive")
      }
    }
  }
}

macro_rules! decl_serde_v5 {
  {
    enum $name:ident ;
  } => {
    decl_serde_v5! {
      base = u8;
      enum $name ;
    }
  };
  {
    base = $basety:ty;
    enum $name:ident ;
  } => {
    impl crate::v5::SerializeV5 for $name {
      fn serialize<'ser>(
        &self,
        ser: &mut crate::v5::AirmashSerializerV5<'ser>
      ) -> ::std::result::Result<(), crate::v5::Error> {
        ser.serialize(&<$basety>::from(*self))
      }
    }

    impl<'de> crate::v5::DeserializeV5<'de> for $name {
      fn deserialize(
        de: &mut crate::v5::AirmashDeserializerV5<'de>
      ) -> ::std::result::Result<Self, crate::v5::Error> {
        use ::std::convert::TryFrom;

        let val: $basety = de.deserialize()?;
        Self::try_from(val)
          .map_err(|_| crate::v5::Error::new(crate::v5::ErrorKind::InvalidEnumValue))
      }
    }
  }
}

macro_rules! decl_enum {
  {
    $(
      $( #[$attr:meta] )*
      $( ##[default = $default:ident] )?
      $( ##[base = $basety:ty ] )?
      $vis:vis enum $name:ident {
        $(
          $( #[$elemattr:meta] )*
          $elem:ident = $value:expr
        ),* $(,)?
      }
    )*
  } => {
    $(
      $( #[$attr] )*
      #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
      #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
      $vis enum $name {
        $(
          $( #[$elemattr] )*
          $elem = $value,
        )*
      }

      impl ::num_traits::FromPrimitive for $name {
        fn from_u64(val: u64) -> Option<Self> {
          Some(match val as _ {
            $( $value => Self::$elem, )*
            _ => return None
          })
        }

        fn from_i64(val: i64) -> Option<Self> {
          Some(match val as _ {
            $( $value => Self::$elem, )*
            _ => return None
          })
        }
      }

      impl ::num_traits::ToPrimitive for $name {
        fn to_u64(&self) -> Option<u64> {
          Some(*self as isize as u64)
        }

        fn to_i64(&self) -> Option<i64> {
          Some(*self as isize as i64)
        }
      }

      decl_enum_from_to!{
        $( default = $default; )?
        enum $name;
      }

      $(
        impl ::std::default::Default for $name {
          fn default() -> Self {
            Self::$default
          }
        }
      )?

      decl_serde_v5! {
        $( base = $basety; )?
        enum $name;
      }
    )*
  };
}
