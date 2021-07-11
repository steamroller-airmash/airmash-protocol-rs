macro_rules! decl_serde {
  {$(
    struct $name:ident {
      $( $field:ident $( => { $ser:ident, $de:ident } )? ),* $(,)?
    }
  )*} => {
    $(
      impl SerializeV5 for $name {
        fn serialize<'ser>(&self, ser: &mut AirmashSerializerV5<'ser>) -> Result {
          #[allow(dead_code)]
          use crate::v5::error::ErrorExt;

          let Self { $( $field, )* } = self;

          $(
            decl_serde!(ser = ser => $field $( { $ser } )?)
              .with_context(stringify!($field))?;
          )*

          Ok(())
        }
      }

      impl<'de> DeserializeV5<'de> for $name {
        fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
          #[allow(dead_code)]
          use crate::v5::error::ErrorExt;

          Ok(Self {
            $(
              $field: decl_serde!(de = de $( { $de } )?)
                .with_context(stringify!($field))?,
            )*
          })
        }
      }
    )*
  };
  { ser = $v:ident => $field:ident } => { $field.serialize($v) };
  // Special cases where the argument type isn't quite as expected
  { ser = $v:ident => $field:ident { serialize_text_small } } => {
    $v.serialize_text_small((***$field).into())
  };
  { ser = $v:ident => $field:ident { serialize_text_large } } => {
    $v.serialize_text_large((***$field).into())
  };
  { ser = $v:ident => $field:ident { serialize_array_small } } => {
    $v.serialize_array_small(&**$field)
  };
  { ser = $v:ident => $field:ident { serialize_array_large } } => {
    $v.serialize_array_large(&**$field)
  };
  { ser = $v:ident => $field:ident { $ser:ident } } => {
    $v.$ser(*$field)
  };
  { de = $v:ident } => { $v.deserialize() };
  { de = $v:ident { $de:ident } } => {
    $v.$de()
  };
}

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

macro_rules! packet_serde {
  {
    enum $name:ident {
      $( $var:ident $( ( $x:ident ) )? ),* $(,)?
    }
  } => {
    impl SerializeV5 for $name {
      fn serialize(&self, ser: &mut AirmashSerializerV5) -> Result {
        use crate::v5::ErrorExt as _;
        
        match self {
          $( $name::$var $( ( $x ) )? => {
            ser.serialize_u8($var::V5_PACKET_NO)?;
            $( 
              ser.serialize($x)
                .with_context(stringify!($name))
                .with_context(stringify!($var))?;
            )?
          }),*
        }

        Ok(())
      }
    }

    impl<'de> DeserializeV5<'de> for $name {
      fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
        use crate::v5::ErrorExt as _;

        match de.deserialize_u8()? {
          $(
            $var::V5_PACKET_NO =>
              Ok($name::$var $( ({
                #[allow(unused_variables)]
                let $x = ();
                de.deserialize()
                   .with_context(stringify!($name))
                   .with_context(stringify!($var))?
              }) )?),
          )*
          _ => Err(super::Error::new(super::ErrorKind::InvalidEnumValue))
        }
      }
    }
  }
}
