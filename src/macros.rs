
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

macro_rules! field_serialize_2 {
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
    ($self:ident, $ser:expr, $name:ident : rotation) => {
        special_field_serialize!($self, $ser, $name : rotation)
    };
    ($self:ident, $ser:expr, $name:ident : healthnergy) => {
        special_field_serialize!($self, $ser, $name : healthnergy)
    };
    ($self:ident, $ser:expr, $name:ident : uint24) => {
        special_field_serialize!($self, $ser, $name : uint24)
    };
    ($self:ident, $ser:expr, $name:ident : coordx) => {
        special_field_serialize!($self, $ser, $name : coordx)
    };
    ($self:ident, $ser:expr, $name:ident : coordy) => {
        special_field_serialize!($self, $ser, $name : coordy)
    };
    ($self:ident, $ser:expr, $name:ident : coord24) => {
        special_field_serialize!($self, $ser, $name : coord24)
    };
    ($self:ident, $ser:expr, $name:ident : regen) => {
        special_field_serialize!($self, $ser, $name : regen)
    };
    ($self:ident, $ser:expr, $name:ident : accel) => {
        special_field_serialize!($self, $ser, $name : accel)
    };
    ($self:ident, $ser:expr, $name:ident : speed) => {
        special_field_serialize!($self, $ser, $name : speed)
    };
    
    ($self:ident, $ser:expr, $name:ident : $type:ty) => {
        $self.$name.serialize($ser)?
    };
}

macro_rules! field_serialize {
    ($self:ident, $de:expr, $name:ident : $type:ident) => {
        field_serialize_2!($self, $de, $name : $type);
    };
    ($self:ident, $de:expr, $name:ident : $type:ident $a:ty ) => {
        field_serialize_2!($self, $de, $name : $type);
    }
}

macro_rules! field_deserialize_2 {
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
    ($de:expr, $name:ident : rotation) => {
        special_field_deserialize!($de, rotation)
    };
    ($de:expr, $name:ident : healthnergy) => {
        special_field_deserialize!($de, healthnergy)
    };
    ($de:expr, $name:ident : uint24) => {
        special_field_deserialize!($de, uint24)
    };
    ($de:expr, $name:ident : coordy) => {
        special_field_deserialize!($de, coordy)
    };
    ($de:expr, $name:ident : coordx) => {
        special_field_deserialize!($de, coordx)
    };
    ($de:expr, $name:ident : coord24) => {
        special_field_deserialize!($de, coord24)
    };
    ($de:expr, $name:ident : regen) => {
        special_field_deserialize!($de, regen)
    };
    ($de:expr, $name:ident : accel) => {
        special_field_deserialize!($de, accel)
    };
    ($de:expr, $name:ident : speed) => {
        special_field_deserialize!($de, speed)
    };

    ($de:expr, $name:ident : $type:ident) => {
        $type::deserialize($de)?
    }
}

macro_rules! field_deserialize {
    ($de:expr, $name:ident : $type:ident) => {
        field_deserialize_2!($de, $name : $type);
    };
    ($de:expr, $name:ident : $type:ident + $a:ty ) => {
        field_deserialize_2!($de, $name : $type);
    }
}

macro_rules! get_field_type {
    (text) => { String };
    (textbig) => { String };
    (array[$subty:ty]) => { ::std::vec::Vec<get_field_type!($subty)> };
    (arraysmall[$subty:ty]) => { ::std::vec::Vec<get_field_type!($subty)> };
    (rotation) => { f32 };
    (healthnergy) => { f32 };
    (uint24) => { u32 };
    (coordy) => { f32 };
    (coordx) => { f32 };
    (coord24) => { f32 };
    (regen) => { f32 };
    (accel) => { f32 };
    (speed) => { f32 };
    
    ($type:ty) => { $type };
}

macro_rules! serde_decl {
    ($(#[$attr:meta])* struct $name:ident { $($field:ident : $type:tt $([ $targs:ty ])*),* }) => {
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
                pub $field: get_field_type!($type $([ $targs ])*),
            )*
        }
    };
}

#[macro_export]
macro_rules! serde_decls {
    {$($(#[$attr:meta])* pub struct $name:ident { $(pub $field:ident : $type:tt $([ $targs:ty ])*),* })*} => {
        $(
            serde_decl!{
                $( #[$attr] )*
                struct $name {
                    $( $field : $type $([ $targs ])* ),*
                }
            }
        )*
    };
}
