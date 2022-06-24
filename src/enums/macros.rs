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

macro_rules! decl_enum_utils {
  {
    [enum $( base($basety:ty) )?]

    $( #[$attr:meta] )*
    $vis:vis enum $name:ident {
      $(
        $( #[$elemattr:meta] )*
        $elem:ident
      ),* $(,)?
    }
  } => {
    $( #[$attr] )*
    #[non_exhaustive]
    $vis enum $name {
      $(
        $( #[$elemattr] )*
        $elem,
      )*

      Unknown(enum_basetype!($($basety)?))
    }
  };
  {
    [enum catchall($catchall:ident) $( base($basety:ty) )?]

    $( #[$attr:meta] )*
    $vis:vis enum $name:ident {
      $(
        $( #[$elemattr:meta] )*
        $elem:ident
      ),* $(,)?
    }
  } => {
    $( #[$attr] )*
    $vis enum $name {
      $(
        $( #[$elemattr] )*
        $elem,
      )*
    }
  };

  {
    match {
      catchall => $expr1:expr,
      default  => $expr2:expr $(,)?
    }
  } => {
    $expr2
  };
  {
    match catchall($catchall:ident) {
      catchall => $expr1:expr,
      default  => $expr2:expr $(,)?
    }
  } => {
    $expr1
  };


}

macro_rules! decl_enum {
  {
    $(
      $( #[$attr:meta] )*
      $( ##[default = $default:ident] )?
      $( ##[base = $basety:ty ] )?
      $( ##[catchall = $catchall:ident ] )?
      $vis:vis enum $name:ident {
        $(
          $( #[$elemattr:meta] )*
          $elem:ident = $value:expr
        ),* $(,)?
      }
    )*
  } => {
    $(
      decl_enum_utils! {
        [enum $( catchall($catchall) )? $( base($basety) )?]

        $( #[$attr] )*
        #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
        $vis enum $name {
          $(
            $( #[$elemattr] )*
            $elem,
          )*
        }
      }

      const _: () = {
        type BaseTy = enum_basetype!($($basety)?);

        impl From<BaseTy> for $name {
          fn from(v: BaseTy) -> Self {
            match v {
              $( $value => Self::$elem, )*
              #[allow(unused_variables, unreachable_patterns)]
              v => decl_enum_utils! {
                match $( catchall($catchall) )? {
                  catchall => { $( Self::$catchall )? },
                  default  => Self::Unknown(v),
                }
              }
            }
          }
        }

        impl From<$name> for BaseTy {
          fn from(v: $name) -> Self {
            match v {
              $( $name::$elem => $value, )*
              #[allow(unused_variables, unreachable_patterns)]
              v => decl_enum_utils! {
                match $( catchall($catchall) )? {
                  catchall => unreachable!(),
                  default  => match v {
                    $name::Unknown(v) => v,
                    _ => unreachable!(),
                  },
                }
              }
            }
          }
        }
      };

      #[cfg(feature = "serde")]
      const _: () = {
        use ::serde::{Serialize, Serializer, Deserialize, Deserializer};
        use ::serde::de::{Visitor, Unexpected, self};
        use ::core::fmt;

        type BaseTy = enum_basetype!($($basety)?);

        const NAME: &'static str = stringify!($name);
        const VARIANTS: &'static [&'static str] = &[
          $( stringify!($elem), )*
        ];

        impl Serialize for $name {
          fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
          where
            S: Serializer,
          {
            if !ser.is_human_readable() {
              return BaseTy::from(*self).serialize(ser);
            }

            match self {
              $( Self::$elem => ser.serialize_str(stringify!($elem)), )*
              #[allow(unused_variables, unreachable_patterns)]
              v => decl_enum_utils! {
                match $( catchall($catchall) )? {
                  catchall => unreachable!(),
                  default  => match v {
                    $name::Unknown(v) => v.serialize(ser),
                    _ => unreachable!(),
                  },
                }
              }
            }
          }
        }

        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
          type Value = $name;

          fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str(concat!("enum ", stringify!($name)))
          }

          fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
          where
            E: de::Error
          {
            use ::core::convert::TryInto;

            let value: BaseTy = value.try_into() //
              .map_err(|_| E::invalid_value(Unexpected::Unsigned(value), &self))?;

            Ok(match value {
              $( $value => Self::Value::$elem, )*
              #[allow(unused_variables)]
              value => decl_enum_utils! {
                match $( catchall($catchall) )? {
                  catchall => { $( Self::Value::$catchall )? },
                  default  => Self::Value::Unknown(value),
                }
              }
            })
          }

          fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
          where
            E: de::Error
          {
            $(
              if crate::util::variant_eq(value.as_bytes(), stringify!($elem)) {
                return Ok(Self::Value::$elem);
              }
            )*

            Err(E::unknown_variant(value, VARIANTS))
          }

          fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
          where
            E: de::Error
          {
            $(
              if crate::util::variant_eq(value, stringify!($elem)) {
                return Ok(Self::Value::$elem);
              }
            )*

            let value = String::from_utf8_lossy(value);
            Err(E::unknown_variant(&value, VARIANTS))
          }
        }

        impl<'de> Deserialize<'de> for $name {
          fn deserialize<D>(de: D) -> Result<Self, D::Error>
          where
            D: Deserializer<'de>,
          {
            match de.is_human_readable() {
              true  => de.deserialize_any(FieldVisitor),
              false => BaseTy::deserialize(de).map(From::from),
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
