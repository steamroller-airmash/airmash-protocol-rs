use serde::*;

impl Serialize for u8 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_u8(*self)
    }
}
impl<'de> Deserialize<'de> for u8 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_u8()
    }
}

impl Serialize for u16 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_u16(*self)
    }
}
impl<'de> Deserialize<'de> for u16 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_u16()
    }
}

impl Serialize for u32 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_u32(*self)
    }
}
impl<'de> Deserialize<'de> for u32 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_u32()
    }
}

impl Serialize for u64 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_u64(*self)
    }
}
impl<'de> Deserialize<'de> for u64 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_u64()
    }
}

impl Serialize for i8 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_i8(*self)
    }
}
impl<'de> Deserialize<'de> for i8 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_i8()
    }
}

impl Serialize for i16 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_i16(*self)
    }
}
impl<'de> Deserialize<'de> for i16 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_i16()
    }
}

impl Serialize for i32 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_i32(*self)
    }
}
impl<'de> Deserialize<'de> for i32 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_i32()
    }
}

impl Serialize for i64 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_i64(*self)
    }
}
impl<'de> Deserialize<'de> for i64 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_i64()
    }
}

impl Serialize for f32 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_f32(*self)
    }
}
impl<'de> Deserialize<'de> for f32 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_f32()
    }
}

impl Serialize for f64 {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_f64(*self)
    }
}
impl<'de> Deserialize<'de> for f64 {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_f64()
    }
}

impl Serialize for bool {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_u8(if *self { 1 } else { 0 })
    }
}
impl<'de> Deserialize<'de> for bool {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match de.deserialize_u8()? {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
}
