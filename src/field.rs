
use std;
use serde;
use serde::de::{Visitor, Error};

struct U8Visitor {}
struct U16Visitor {}

impl<'de> Visitor<'de> for U8Visitor {
	type Value = u8;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a u8")
	}

	fn visit_u8<E>(self, value: Self::Value) -> Result<Self::Value, E>
		where E: serde::de::Error
	{
		Ok(value)
	}
}
impl<'de> Visitor<'de> for U16Visitor {
	type Value = u16;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a u16")
	}

	fn visit_u16<E>(self, value: Self::Value) -> Result<Self::Value, E>
		where E: serde::de::Error
	{
		Ok(value)
	}
}

#[derive(Serialize)]
struct StrRepr<'a> {
	len: u16,
	bytes: Vec<u8>
}

impl<'b, 'a, 'de: 'a> Visitor<'de> for &'b mut StrRepr<'a> {
	type Value = ();

	fn visit_u8<E>(self, value: u8) -> Result<(), E>
		where E: serde::de::Error
	{
		if self.len == 0 {
			self.len = value as u16;
		}
		else {
			self.bytes.push(value);
		}
	}

	fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
		where E: serde::de::Error
	{
		if self.len == 0 {
			self.len = value;
			Ok(())
		}
		else {
			Err(E::construct("Error"))
		}
	}
}

pub mod text {
	use serde::{Serialize, Serializer, Deserializer };
	use field::*;


	pub fn serialize<S>(val: &String, s: S) -> Result<S::Ok, S::Error>
		where S: Serializer + Copy
	{
		let bytes = val.as_bytes();

		if bytes.len() > 255 {
			panic!("String length greater than format allows!");
		}

		StrRepr{ len: bytes.len() as u8, bytes }.serialize(s)
	}

	pub fn deserialize<'de, D>(de: D) -> Result<String, D::Error>
		where D: Deserializer<'de> + Copy
	{
		let parts = de.deserialize_tuple(U8Visitor{})? as usize;
		let mut bytes = vec![];

		for _ in 0..len {
			bytes.push(de.deserialize_u8(U8Visitor{})?);
		}

		Ok(String::from_utf8(bytes).unwrap())
	}
}

/*pub mod textbig {
	use serde::{Serializer, Deserializer};
	use field::*;

	pub fn serialize<S>(val: &String, s: &mut S) -> Result<S::Ok, S::Error>
		where S: Serializer
	{
		let bytes = val.as_bytes();

		if bytes.len() > 65535 {
			panic!("String length greater than format allows!");
		}

		s.serialize_u16(bytes.len() as u16)?;
		s.serialize_bytes(bytes)
	}
	
	pub fn deserialize<'de, D>(de: D) -> Result<String, D::Error>
		where D: Deserializer<'de>
	{
		let len = de.deserialize_u16(U16Visitor{})? as usize;
		let bytes = vec![];

		for _ in 0..len {
			bytes.push(de.deserialize_u8(U8Visitor{})?);
		}

		Ok(String::from_utf8(bytes).unwrap())
	}
}*/

/*pub mod array {
	use serde::{Serialize, Deserialize, Serializer, Deserializer};
	use field::*;

	pub fn serialize<S, T>(arr: &Vec<T>, s: &mut S) -> Result<S::Ok, S::Error>
		where S: Serializer,
		      T: Serialize
	{
		if arr.len() > 255 {
			panic!("Array length greater than format allows!");
		}

		let ok = s.serialize_u8(arr.len() as u8)?;
		
		for val in arr {
			val.serialize(s)?;
		}

		return Ok(ok);
	}
	
	pub fn deserialize<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
		where D: Deserializer<'de>,
		      T: Deserialize<'de>
	{
		let len = de.deserialize_u8(U16Visitor{})? as usize;
		let arr = vec![];

		for _ in 0..len {
			arr.push(T::deserialize(de)?);
		}

		Ok(arr)
	}

}*/
