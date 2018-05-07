
use serde::*;

pub type SerResult<S> = Result<<S as Serializer>::Ok, <S as Serializer>::Error>;

pub mod textbig {
	use serde::*;
	use field::*;

	pub fn serialize<S>(val: &str, ser: &mut S) -> SerResult<S>
		where S: Serializer
	{
		let bytes = val.as_bytes();

		ser.serialize_u16(bytes.len() as u16)?;
		ser.serialize_bytes(bytes)
	}
	pub fn deserialize<'de, D>(de: &mut D) -> Result<String, D::Error> 
		where D: Deserializer<'de>
	{
		let len = de.deserialize_u16()?;
		Ok(de.deserialize_str(len as usize)?.to_string())
	}
}

pub mod text {
	use serde::*;
	use field::*;

	pub fn serialize<S>(val: &str, ser: &mut S) -> SerResult<S>
		where S: Serializer
	{
		let bytes = val.as_bytes();

		ser.serialize_u8(bytes.len() as u8)?;
		ser.serialize_bytes(bytes)
	}
	pub fn deserialize<'de, D>(de: &mut D) -> Result<String, D::Error> 
		where D: Deserializer<'de>
	{
		let len = de.deserialize_u8()?;
		Ok(de.deserialize_str(len as usize)?.to_string())
	}
}

pub mod array {
	use serde::*;
	use field::*;
	use std::vec::Vec;

	pub fn serialize<S, T>(arr: &Vec<T>, ser: &mut S) -> SerResult<S>
		where S: Serializer,
			  T: Serialize
	{
		let s = ser.serialize_u16(arr.len() as u16)?;

		for val in arr {
			val.serialize(ser)?;
		}

		Ok(s)
	}
	pub fn deserialize<'de, D, T>(de: &mut D) -> Result<Vec<T>, D::Error> 
		where D: Deserializer<'de>,
		      T: Deserialize<'de>
	{
		let len = de.deserialize_u16()?;
		let mut vec = vec![];

		for _ in 0..len {
			vec.push(T::deserialize(de)?);
		}

		Ok(vec)
	}
}

pub mod arraysmall {
	use serde::*;
	use field::*;

	pub fn serialize<S, T>(arr: &[T], ser: &mut S) -> SerResult<S>
		where S: Serializer,
			  T: Serialize
	{
		let s = ser.serialize_u8(arr.len() as u8)?;

		for val in arr {
			val.serialize(ser)?;
		}

		Ok(s)
	}
	pub fn deserialize<'de, D, T>(de: &mut D) -> Result<Vec<T>, D::Error> 
		where D: Deserializer<'de>,
		      T: Deserialize<'de>
	{
		let len = de.deserialize_u8()?;
		let mut vec = vec![];

		for _ in 0..len {
			vec.push(T::deserialize(de)?);
		}

		Ok(vec)
	}
}

