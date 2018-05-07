
use std::mem;
use std::str;

use serde::{self, Deserialize};
use error::{Error, Result};

pub struct Deserializer<'a> {
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
}

impl<'de> serde::Deserializer<'de> for Deserializer<'de> {
	type Error = Error;

	fn deserialize_i8 (&mut self) -> Result<i8> {
		Ok(self.deserialize_u8()? as i8)
	}
	fn deserialize_i16(&mut self) -> Result<i16> {
		Ok(self.deserialize_u16()? as i16)
	}
	fn deserialize_i32(&mut self) -> Result<i32> {
		Ok(self.deserialize_u32()? as i32)
	}
	fn deserialize_i64(&mut self) -> Result<i64> {
		Ok(self.deserialize_u64()? as i64)
	}

	fn deserialize_u8 (&mut self) -> Result<u8> {
		if self.bytes.len() == 0 {
			return Err(Error::Eof)
		}

		let b = self.bytes[0];
		self.bytes = &self.bytes[1..];
		Ok(b)
	}
	fn deserialize_u16(&mut self) -> Result<u16> {
		let hi = self.deserialize_u8()?;
		let lo = self.deserialize_u8()?;

		Ok(((hi as u16) << 8) | (lo as u16))
	}
	fn deserialize_u32(&mut self) -> Result<u32> {
		let hi = self.deserialize_u16()?;
		let lo = self.deserialize_u16()?;

		Ok(((hi as u32) << 16) | (lo as u32))
	}
	fn deserialize_u64(&mut self) -> Result<u64> {
		let hi = self.deserialize_u32()?;
		let lo = self.deserialize_u32()?;

		Ok(((hi as u64) << 32) | (lo as u64))
	}

	fn deserialize_f32(&mut self) -> Result<f32> {
		Ok(unsafe { mem::transmute::<u32, f32>(self.deserialize_u32()?) })
	}
	fn deserialize_f64(&mut self) -> Result<f64> {
		Ok(unsafe { mem::transmute::<u64, f64>(self.deserialize_u64()?) })
	}

	fn deserialize_unit(&mut self) -> Result<()> {
		Ok(())
	}
	fn deserialize_bytes(&mut self, len: usize) -> Result<&'de [u8]> {
		if self.bytes.len() < len {
			return Err(Error::Eof);
		}
		
		let slice = &self.bytes[0..len];
		self.bytes = &self.bytes[len..];
		Ok(slice)
	}
	fn deserialize_str(&mut self, len: usize) -> Result<&'de str> {
		let bytes = self.deserialize_bytes(len)?;
		
		match str::from_utf8(bytes) {
			Ok(val) => Ok(val),
			Err(e) => Err(Error::Utf8Error(e))
		}
	}
}