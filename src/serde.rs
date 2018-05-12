
pub use ser::Serializer;
pub use de::Deserializer;
pub use error::Result;

pub trait Serialize {
    fn serialize(&self, ser: &mut Serializer) -> Result<()>;
}
pub trait Deserialize<'de> {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<Self>
    where
        Self: Sized;
}
