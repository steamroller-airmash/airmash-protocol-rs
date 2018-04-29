
use std::str;
use std::mem;

use serde;
use serde::de::{self, Deserialize, DeserializeSeed, SeqAccess, Visitor};

use error::{Error, Result};

struct Deserializer<'a> {
	pub bytes: &'a [u8]
}

pub fn from_bytes<'a, T>(b: &'a [u8]) -> Result<T>
	where T: Deserialize<'a>
{
	let mut deserializer = Deserializer::from_bytes(b);
	let t = T::deserialize(&mut deserializer)?;

	if deserializer.bytes.is_empty() {
		Ok(t)
	}
	else {
		Err(Error::TrailingBytes)
	}
}

impl<'a> Deserializer<'a> {
	pub fn from_bytes(bytes: &'a [u8]) -> Self {
		return Self { bytes };
	}

	fn get_byte(&mut self) -> Result<u8> {
		if self.bytes.is_empty() {
			return Err(Error::Eof);
		}

		let b = self.bytes[0];
		self.bytes = &self.bytes[1..];
		return Ok(b);
	}
	fn get_u16(&mut self) -> Result<u16> {
		Ok(((self.get_byte()? as u16) << 8) | (self.get_byte()? as u16))
	}
	fn get_u32(&mut self) -> Result<u32> {
		Ok(((self.get_u16()? as u32) << 16) | (self.get_u16()? as u32))
	}
	fn get_u64(&mut self) -> Result<u64> {
		Ok(((self.get_u32()? as u64) << 32) | (self.get_u32()? as u64))
	}
}

impl<'a, 'de: 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
	type Error = Error;

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de> 
	{
		visitor.visit_u8(self.get_byte()?)
	}
	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_u16(self.get_u16()?)
	}
	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_u32(self.get_u32()?)
	}
	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_u64(self.get_u64()?)
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_i8(self.get_byte()? as i8)
	}
	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_i16(self.get_u16()? as i16)
	}
	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_i32(self.get_u32()? as i32)
	}
	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_i64(self.get_u64()? as i64)
	}

	fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		struct Access<'b, 'de: 'b> {
			deserializer: &'b mut Deserializer<'de>,
			len: usize
		}

		impl<'b, 'de: 'b> SeqAccess<'de> for Access<'b, 'de> {
			type Error = Error;

			fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
				where T: serde::de::DeserializeSeed<'de>
			{
				if self.len > 0 {
					self.len -= 1;
					let value = try!(DeserializeSeed::deserialize(
						seed,
						&mut *self.deserializer
					));
					Ok(Some(value))
				}
				else {
					Ok(None)
				}
			}	

			fn size_hint(&self) -> Option<usize> {
				Some(self.len)
			}
		}

		visitor.visit_seq(Access {
			deserializer: self,
			len: len
		})
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		fields: &'static [&'static str],
		visitor: V
	) -> Result<V::Value> 
		where V: Visitor<'de>
	{
		self.deserialize_tuple(fields.len(), visitor)
	}
	fn deserialize_tuple_struct<V>(
		self, 
		_name: &'static str, 
		len: usize,
		visitor: V
	) -> Result<V::Value>
		where V: Visitor<'de>
	{
		self.deserialize_tuple(len, visitor)
	}
	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V
	) -> Result<V::Value>
		where V: Visitor<'de>
	{
		self.deserialize_tuple(1, visitor)
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_unit()
	}
	fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		visitor.visit_unit()
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		let v = unsafe{ mem::transmute::<u32, f32>(self.get_u32()?) };
		visitor.visit_f32(v)
	}
	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		let v = unsafe{ mem::transmute::<u64, f64>(self.get_u64()?) };
		visitor.visit_f64(v)
	}

	// Unimplemented Methods
	fn deserialize_any<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_bool<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_char<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_map<V>(self, _: V) -> Result<V::Value> 
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_str<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_string<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_bytes<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_byte_buf<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_option<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_seq<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_enum<V>(
		self, 
		_name: &'static str,
		_variants: &'static [&'static str],
		_: V
	) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_identifier<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
	fn deserialize_ignored_any<V>(self, _: V) -> Result<V::Value>
		where V: Visitor<'de>
	{
		unimplemented!();
	}
}
