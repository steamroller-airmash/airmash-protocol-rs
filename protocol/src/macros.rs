
macro_rules! special_field_serialize {
    ($self:ident, $ser:expr, $name:ident : $type:ident) => {
        ::field::$type::serialize(&$self.$name, $ser)?
    }
}
macro_rules! special_field_deserialize {
    ($de:expr, $type:ident) => {
        ::field::$type::deserialize($de)?
    };
}

macro_rules! field_serialize {
    ($self:ident, $ser:expr, $name:ident : text) => {
        special_field_serialize!($self, $ser, $name : text)
    };
    ($self:ident, $ser:expr, $name:ident : textbig) => {
        special_field_serialize!($self, $ser, $name : textbig)
    };
    ($self:ident, $ser:expr, $name:ident : array) => {
        special_field_serialize!($self, $ser, $name : array)
    };
    ($self:ident, $ser:expr, $name:ident : arraysmall) => {
        special_field_serialize!($self, $ser, $name : arraysmall)
    };
    
    ($self:ident, $ser:expr, $name:ident : $type:ty) => {
        $self.$name.serialize($ser)?
    };
}

macro_rules! field_deserialize {
    ($de:expr, $name:ident : text) => {
        special_field_deserialize!($de, text)
    };
    ($de:expr, $name:ident : textbig) => {
        special_field_deserialize!($de, textbig)
    };
    ($de:expr, $name:ident : array) => {
        special_field_deserialize!($de, array)
    };
    ($de:expr, $name:ident : arraysmall) => {
        special_field_deserialize!($de, arraysmall)
    };

    ($de:expr, $name:ident : $type:ident) => {
        $type::deserialize($de)?
    }
}

macro_rules! get_field_type {
    (text) => { String };
    (textbig) => { String };
    (array<$subty:ty>) => { ::std::vec::Vec<get_field_type!($subty)> };
    (arraysmall<$subty:ty>) => { ::std::vec::Vec<get_field_type!($subty)> };
    ($type:ty) => { $type };
}

macro_rules! serde_decl {
    ($(#[$attr:meta])* struct $name:ident { $($field:ident : $type:tt),* }) => {
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error> 
                where S: ::serde::Serializer
            {
                #[allow(unused_imports)]
                use ::serde::*;

                // This is harmless, since unit 
                // serializes to nothing
                let _result = ser.serialize_unit()?;

                $(
                    let _result = field_serialize!(self, ser, $field : $type);
                )*

                Ok(_result)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(_de: &mut D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>
            {
                #[allow(unused_imports)]
                use ::serde::*;

                Ok(Self {
                    $(
                        $field: field_deserialize!(_de, $field : $type),
                    )*
                })
            }
        }
        
        $(#[$attr])*
        pub struct $name {
            $(
                pub $field: get_field_type!($type),
            )*
        }
    };
}

#[macro_export]
macro_rules! serde_decls {
    {$($(#[$attr:meta])* pub struct $name:ident { $(pub $field:ident : $type:tt),* })*} => {
        $(
            serde_decl!{
                $( #[$attr] )*
                struct $name {
                    $( $field : $type ),*
                }
            }
        )*
    };
}
