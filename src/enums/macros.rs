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

macro_rules! enum_basetype {
  ($basety:ty) => {
    $basety
  };
  () => {
    u8
  };
}

#[allow(unused_macros)]
macro_rules! dummy_count {
  ($($tt:tt)*) => {
    1
  };
}

#[allow(unused_macros)]
macro_rules! enum_variant_serialize_decl {
    {
      [$index:expr] $name:ident => $elem:ident $( $rest:ident )*;

      match $self:ident => $ser:ident {
        $( $pat:pat => $expr:expr ),* $(,)?
      }
    } => {
      enum_variant_serialize_decl! {
        [$index + 1] $name => $( $rest )* ;

        match $self => $ser {
          Self::$elem => $ser.serialize_unit_variant(
            stringify!($name),
            $index,
            stringify!($elem),
          ),
          $( $pat => $expr ),*
        }
      }
    };
    {
      [$index:expr] $name:ident => ;

      match $self:ident => $ser:ident {
        $( $pat:pat => $expr:expr ),* $(,)?
      }
    } => {

      match $self {
        $( $pat => $expr, )*
        Self::Unknown(val) => {
          let mut ser = $ser.serialize_tuple_variant(
            stringify!($name),
            $index,
            "Unknown",
            1
          )?;
          ser.serialize_field(val)?;
          ser.end()
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
      $vis enum $name {
        $(
          $( #[$elemattr] )*
          $elem,
        )*

        Unknown(enum_basetype!($($basety)?))
      }

      const _: () = {
        type BaseTy = enum_basetype!($($basety)?);

        impl From<BaseTy> for $name {
          fn from(v: BaseTy) -> Self {
            match v {
              $( $value => Self::$elem, )*
              v => Self::Unknown(v)
            }
          }
        }

        impl From<$name> for BaseTy {
          fn from(v: $name) -> Self {
            match v {
              $( $name::$elem => $value, )*
              $name::Unknown(v) => v,
            }
          }
        }
      };

      #[cfg(feature = "serde")]
      const _: () = {
        use std::fmt;

        use serde::de::{Unexpected, Visitor};
        use serde::ser::SerializeTupleVariant;
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        type BaseTy = enum_basetype!($($basety)?);

        impl Serialize for $name {
          fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
          where
            S: Serializer,
          {
            if ser.is_human_readable() {
              enum_variant_serialize_decl! {
                [0u32] $name => $( $elem )*;

                match self => ser {}
              }
            } else {
              BaseTy::serialize(&BaseTy::from(*self), ser)
            }
          }
        }

        impl<'de> Deserialize<'de> for $name {
          fn deserialize<D>(de: D) -> Result<Self, <D as Deserializer<'de>>::Error>
          where
            D: Deserializer<'de>,
          {
            struct TyVisitor;

            impl<'de> Visitor<'de> for TyVisitor {
              type Value = $name;

              fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(
                  fmt,
                  "an enum value (either a string or a {})",
                  None
                    $( .or_else(|| Some(stringify!($basety))) )?
                    .unwrap_or("u8")
                )
              }

              fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
              where
                E: serde::de::Error,
              {
                match v {
                  $( stringify!($elem) => Ok($name::$elem), )*
                  _ => Err(E::invalid_value(Unexpected::Str(v), &self)),
                }
              }

              fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
              where
                E: serde::de::Error
              {
                use std::convert::TryFrom;

                match BaseTy::try_from(v) {
                  Ok(v) => Ok($name::from(v)),
                  Err(_) => Err(E::invalid_value(
                    Unexpected::Unsigned(v),
                    &self
                  ))
                }
              }
            }

            de.deserialize_enum(
              stringify!($name),
              &[ $( stringify!($elem) ),* ],
              TyVisitor
            )
          }
        }
      };

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
