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
          $elem,
        )*

        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
