pub trait Serializer {
    type Ok;
    type Error;

    fn serialize_i8(&mut self, v: i8) -> Result<Self::Ok, Self::Error>;
    fn serialize_i16(&mut self, v: i16) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(&mut self, v: i32) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(&mut self, v: i64) -> Result<Self::Ok, Self::Error>;

    fn serialize_u8(&mut self, v: u8) -> Result<Self::Ok, Self::Error>;
    fn serialize_u16(&mut self, v: u16) -> Result<Self::Ok, Self::Error>;
    fn serialize_u32(&mut self, v: u32) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(&mut self, v: u64) -> Result<Self::Ok, Self::Error>;

    fn serialize_f32(&mut self, v: f32) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(&mut self, v: f64) -> Result<Self::Ok, Self::Error>;

    fn serialize_bool(&mut self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit(&mut self) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(&mut self, bytes: &[u8]) -> Result<Self::Ok, Self::Error>;
}

pub trait Deserializer<'de> {
    type Error;

    fn deserialize_i8(&mut self) -> Result<i8, Self::Error>;
    fn deserialize_i16(&mut self) -> Result<i16, Self::Error>;
    fn deserialize_i32(&mut self) -> Result<i32, Self::Error>;
    fn deserialize_i64(&mut self) -> Result<i64, Self::Error>;

    fn deserialize_u8(&mut self) -> Result<u8, Self::Error>;
    fn deserialize_u16(&mut self) -> Result<u16, Self::Error>;
    fn deserialize_u32(&mut self) -> Result<u32, Self::Error>;
    fn deserialize_u64(&mut self) -> Result<u64, Self::Error>;

    fn deserialize_f32(&mut self) -> Result<f32, Self::Error>;
    fn deserialize_f64(&mut self) -> Result<f64, Self::Error>;

    fn deserialize_unit(&mut self) -> Result<(), Self::Error>;
    fn deserialize_bytes(&mut self, len: usize) -> Result<&'de [u8], Self::Error>;
    fn deserialize_str(&mut self, len: usize) -> Result<&'de str, Self::Error>;
}

pub trait Serialize {
    fn serialize<S>(&self, ser: &mut S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
pub trait Deserialize<'de> {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
        Self: Sized;
}
