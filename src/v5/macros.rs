#[macro_export]
macro_rules! decl_serde {
  {$(
    struct $name:ident {
      $( $field:ident $( => { $ser:ident, $de:ident } )? ),* $(,)?
    }
  )*} => {
    $(
      impl SerializeV5 for $name {
        fn serialize<'ser>(&self, ser: &mut AirmashSerializerV5<'ser>) -> Result {
          let Self { $( $field, )* } = self;

          $(
            decl_serde!(ser = ser => $field $( { $ser } )?);
          )*

          Ok(())
        }
      }

      impl<'de> DeserializeV5<'de> for $name {
        fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
          Ok(Self {
            $(
              $field: decl_serde!(de = de $( { $de } )?),
            )*
          })
        }
      }
    )*
  };
  { ser = $v:ident => $field:ident } => { $field.serialize($v)?; };
  { ser = $v:ident => $field:ident { $ser:ident } } => {
    $v.$ser($field.as_ref())?;
  };
  { de = $v:ident } => { $v.deserialize()? };
  { de = $v:ident { $de:ident } } => {
    $v.$de()?
  };
}

#[macro_export]
macro_rules! decl_consts {
  {
    $( const $name:ident = $value:expr; )*
  } => {
    $(
      impl $name {
        pub(crate) const V5_PACKET_NO: u8 = $value;
      }
    )*
  }
}

#[macro_export]
macro_rules! packet_serde {
  {
    enum $name:ident {
      $( $var:ident $( ( $x:ident ) )? ),* $(,)?
    }
  } => {
    impl SerializeV5 for $name {
      fn serialize(&self, ser: &mut AirmashSerializerV5) -> Result {
        match self {
          $( $name::$var $( ( $x ) )? => {
            ser.serialize_u8($var::V5_PACKET_NO)?;
            $( ser.serialize($x)?; )?
          }),*
        }
      }
    }

    impl<'de> DeserializeV5<'de> for $name {
      fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
        match de.deserialize_u8()? {
          $(
            $var::V5_PACKET_NO =>
              Ok($name::$var $( ({
                #[allow(unused_variables)]
                let $x = ();
                de.deserialize()
                   .with_context(stringify!($name::$var))?
              }) )?),
          )*
          _ => Err(super::Error::new(super::ErrorKind::InvalidEnumValue))
        }
      }
    }
  }
}
