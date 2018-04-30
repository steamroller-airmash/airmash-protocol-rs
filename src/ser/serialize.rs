
use serde::{self, Serialize};
use error::{Error, Result};

use std::vec::Vec;
use std::mem;

pub struct Serializer {
	output: Vec<u8>
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
	where T: Serialize
{
	let mut serializer = Serializer { output: vec![] };
	value.serialize(&mut serializer)?;
	Ok(serializer.output)
}

impl serde::Serializer for Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_i8(&mut self, v: i8) -> Result<()> {
		self.serialize_u8(v as u8)
	}
	fn serialize_i16(&mut self, v: i16) -> Result<()> {
		self.serialize_u16(v as u16)
	}
	fn serialize_i32(&mut self, v: i32) -> Result<()> {
		self.serialize_u32(v as u32)
	}
	fn serialize_i64(&mut self, v: i64) -> Result<()> {
		self.serialize_u64(v as u64) 
	}

	fn serialize_u8(&mut self, v: u8) -> Result<()> {
		self.output.push(v);
		Ok(())
	}
	fn serialize_u16(&mut self, v: u16) -> Result<()> {
		self.serialize_u8((v >> 8) as u8)?;
		self.serialize_u8(v as u8)
	}
	fn serialize_u32(&mut self, v: u32) -> Result<()> {
		self.serialize_u16((v >> 16) as u16)?;
		self.serialize_u16(v as u16)
	}
	fn serialize_u64(&mut self, v: u64) -> Result<()> {
		self.serialize_u32((v >> 32) as u32)?;
		self.serialize_u32(v as u32)
	}

	fn serialize_f32(&mut self, v: f32) -> Result<()> {
		self.serialize_u32(unsafe{ mem::transmute::<f32, u32>(v) })
	}
	fn serialize_f64(&mut self, v: f64) -> Result<()> {
		self.serialize_u64(unsafe{ mem::transmute::<f64, u64>(v) })
	}

	fn serialize_unit(&mut self) -> Result<()> {
		Ok(())
	}
	// Must have a size called separately beforehand
	fn serialize_bytes(&mut self, v: &[u8]) -> Result<()> {
		self.output.extend_from_slice(v);
		Ok(())
	}
	fn serialize_bool(&mut self, v: bool) -> Result<()> {
		self.serialize_u8(if v { 1 } else { 0 })
	}
}
