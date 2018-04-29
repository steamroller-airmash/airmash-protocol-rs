
use serde::ser::{self, Serialize};
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

impl<'a> ser::Serializer for &'a mut Serializer {
	type Ok = ();
	type Error = Error;
	
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

	fn serialize_i8(self, v: i8) -> Result<()> {
		self.serialize_u8(v as u8)
	}
	fn serialize_i16(self, v: i16) -> Result<()> {
		self.serialize_u16(v as u16)
	}
	fn serialize_i32(self, v: i32) -> Result<()> {
		self.serialize_u32(v as u32)
	}
	fn serialize_i64(self, v: i64) -> Result<()> {
		self.serialize_u64(v as u64) 
	}

	fn serialize_u8(self, v: u8) -> Result<()> {
		self.output.push(v);
		Ok(())
	}
	fn serialize_u16(self, v: u16) -> Result<()> {
		self.serialize_u8((v >> 8) as u8)?;
		self.serialize_u8(v as u8)
	}
	fn serialize_u32(self, v: u32) -> Result<()> {
		self.serialize_u16((v >> 16) as u16)?;
		self.serialize_u16(v as u16)
	}
	fn serialize_u64(self, v: u64) -> Result<()> {
		self.serialize_u32((v >> 32) as u32)?;
		self.serialize_u32(v as u32)
	}

	fn serialize_f32(self, v: f32) -> Result<()> {
		self.serialize_u32(unsafe{ mem::transmute::<f32, u32>(v) })
	}
	fn serialize_f64(self, v: f64) -> Result<()> {
		self.serialize_u64(unsafe{ mem::transmute::<f64, u64>(v) })
	}

	fn serialize_unit(self) -> Result<()> {
		Ok(())
	}
	fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
		self.serialize_unit()
	}
	fn serialize_unit_variant(
		self,
		_name: &'static str, 
		_index: u32,
		_variant: &'static str
	) -> Result<()>	{
		self.serialize_unit()
	}
	fn serialize_newtype_struct<T: ?Sized + Serialize>(
		self, 
		_name: &'static str,
		v: &T
	) -> Result<()> {
		v.serialize(self)
	}
	fn serialize_newtype_variant<T: ?Sized + Serialize>(
		self,
		_name: &'static str,
		_idx: u32,
		_variant: &'static str,
		v: &T
	) -> Result<()> {
		v.serialize(self)
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self> {
		Ok(self)
	}
	fn serialize_tuple_struct(
		self, 
		_: &'static str,
		_len: usize
	) -> Result<Self> {
		Ok(self)
	}
	fn serialize_tuple_variant(
		self,
		_variant: &'static str,
		_idx: u32,
		_name: &'static str,
		_len: usize
	) -> Result<Self> {
		Ok(self)
	}
	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize
	) -> Result<Self> {
		Ok(self)
	}
	fn serialize_struct_variant(
		self,
		_variant: &'static str,
		_idx: u32,
		_name: &'static str,
		_len: usize
	) -> Result<Self> {
		Ok(self)
	}

	// Unimplemented methods
	fn serialize_bool(self, _: bool) -> Result<()> {
		unimplemented!();
	}
	fn serialize_char(self, _: char) -> Result<()> {
		unimplemented!();
	}
	fn serialize_str(self, _: &str) -> Result<()> {
		unimplemented!();
	}
	fn serialize_bytes(self, _: &[u8]) -> Result<()> {
		unimplemented!();
	}
	fn serialize_none(self) -> Result<()> {
		unimplemented!();
	}
	fn serialize_some<T: ?Sized>(self, _:&T) -> Result<()> {
		unimplemented!();
	}
	fn serialize_seq(self, _: Option<usize>) -> Result<Self> {
		unimplemented!();
	}
	fn serialize_map(self, _: Option<usize>) -> Result<Self> {
		unimplemented!();
	}
}

impl <'a> ser::SerializeSeq for &'a mut Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_element<T: ?Sized>(&mut self, v: &T) -> Result<()>
		where T: Serialize
	{
		v.serialize(&mut **self)
	}
	fn end(self) -> Result<()> {
		Ok(())
	}
}
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_field<T: ?Sized>(
		&mut self,
		_name: &'static str,
		v: &T
	) -> Result<()> 
		where T: Serialize
	{
		v.serialize(&mut **self)
	}
	fn end(self) -> Result<()> {
		Ok(())
	}
}
impl<'a> ser::SerializeStruct for &'a mut Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_field<T: ?Sized>(
		&mut self, 
		_name: &'static str,
		v: &T
	) -> Result<()> 
		where T: Serialize
	{
		v.serialize(&mut **self)
	}
	fn end(self) -> Result<()> {
		Ok(())
	}
}
impl<'a> ser::SerializeMap for &'a mut Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<()> 
		where T: Serialize
	{
		unimplemented!();
	}
	fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<()>
		where T: Serialize
	{
		unimplemented!();
	}

	fn end(self) -> Result<()> {
		unimplemented!();
	}
}
impl <'a> ser::SerializeTupleVariant for &'a mut Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_field<T: ?Sized>(&mut self, v: &T) -> Result<()> 
		where T: Serialize
	{
		v.serialize(&mut **self)
	}
	fn end(self) -> Result<()> {
		Ok(())
	}
}
impl<'a> ser::SerializeTuple for &'a mut Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_element<T: ?Sized>(&mut self, v: &T) -> Result<()> 
		where T: Serialize
	{
		v.serialize(&mut **self)
	}
	fn end(self) -> Result<()> {
		Ok(())
	}
}
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
	type Ok = ();
	type Error = Error;

	fn serialize_field<T: ?Sized>(&mut self, v: &T) -> Result<()> 
		where T: Serialize
	{
		v.serialize(&mut **self)
	}
	fn end(self) -> Result<()> {
		Ok(())
	}
}
