use super::Result;
use crate::v5::{Error, ErrorExt as _, ErrorKind};
use crate::Vector2;
use bstr::{BStr, BString, ByteSlice};

struct ScalarSpec {
  shift: i32,
  mult: f32,
}

impl ScalarSpec {
  pub const fn new(shift: i32, mult: f32) -> Self {
    Self { shift, mult }
  }

  fn de(&self, de: &mut AirmashDeserializerV5) -> Result<f32> {
    Ok((((de.deserialize_u16()? as i32) - self.shift) as f32) * (1.0 / self.mult))
  }
  fn ser(&self, ser: &mut AirmashSerializerV5, value: f32) -> Result {
    ser.serialize_u16(((value * self.mult) as i32 + self.shift).clamp(0, u16::MAX as i32) as u16)
  }

  fn de_u8(&self, de: &mut AirmashDeserializerV5) -> Result<f32> {
    Ok((((de.deserialize_u8()? as i32) - self.shift) as f32) * (1.0 / self.mult))
  }
  fn ser_u8(&self, ser: &mut AirmashSerializerV5, value: f32) -> Result {
    ser.serialize_u8(((value * self.mult) as i32 + self.shift).clamp(0, u8::MAX as i32) as u8)
  }
}

const ACCEL_SPEC: ScalarSpec = ScalarSpec::new(32768, 32768.0);
const SPEED_SPEC: ScalarSpec = ScalarSpec::new(32768, 1638.4);
const COORD24_SPEC: ScalarSpec = ScalarSpec::new(8388608, 512.0);
const COORDX_SPEC: ScalarSpec = ScalarSpec::new(32768, 2.0);
const COORDY_SPEC: ScalarSpec = ScalarSpec::new(32768, 4.0);
const ENERGY_SPEC: ScalarSpec = ScalarSpec::new(0, 255.0);
const REGEN_SPEC: ScalarSpec = ScalarSpec::new(32768, 1.0e6);
const ROTATION_SPEC: ScalarSpec = ScalarSpec::new(0, 6553.6);

pub struct AirmashSerializerV5<'ser> {
  data: &'ser mut Vec<u8>,
}

impl<'ser> AirmashSerializerV5<'ser> {
  pub fn new(data: &'ser mut Vec<u8>) -> Self {
    Self { data }
  }

  pub fn serialize<T: SerializeV5>(&mut self, value: &T) -> Result {
    value.serialize(self)
  }

  pub fn serialize_bytes(&mut self, bytes: &[u8]) -> Result {
    self.data.extend_from_slice(bytes);
    Ok(())
  }

  pub fn serialize_u8(&mut self, value: u8) -> Result {
    self.serialize_bytes(&[value])
  }
  pub fn serialize_u16(&mut self, value: u16) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }
  pub fn serialize_u32(&mut self, value: u32) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }
  pub fn serialize_u64(&mut self, value: u64) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }
  pub fn serialize_u128(&mut self, value: u128) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }

  pub fn serialize_i8(&mut self, value: i8) -> Result {
    self.serialize_u8(value as u8)
  }
  pub fn serialize_i16(&mut self, value: i16) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }
  pub fn serialize_i32(&mut self, value: i32) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }
  pub fn serialize_i64(&mut self, value: i64) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }
  pub fn serialize_i128(&mut self, value: i128) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }

  pub fn serialize_f32(&mut self, value: f32) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }
  pub fn serialize_f64(&mut self, value: f64) -> Result {
    self.serialize_bytes(&value.to_le_bytes())
  }

  pub fn serialize_u24(&mut self, value: u32) -> Result {
    self.serialize_bytes(&value.to_le_bytes()[..3])
  }
  pub fn serialize_bool(&mut self, value: bool) -> Result {
    self.serialize_u8(if value { 1 } else { 0 })
  }

  pub fn serialize_array_small<T>(&mut self, data: &[T]) -> Result
  where
    T: SerializeV5,
  {
    if data.len() > u8::MAX as usize {
      return Err(Error::new(ErrorKind::ArraySizeTooLarge));
    }

    self.serialize_u8(data.len() as u8)?;

    for elem in data {
      self.serialize(elem).with_context("<array element>")?;
    }

    Ok(())
  }
  pub fn serialize_array_large<T>(&mut self, data: &[T]) -> Result
  where
    T: SerializeV5,
  {
    if data.len() > u16::MAX as usize {
      return Err(Error::new(ErrorKind::ArraySizeTooLarge));
    }

    self.serialize_u16(data.len() as u16)?;

    for elem in data {
      self.serialize(elem).with_context("<array element>")?;
    }

    Ok(())
  }

  pub fn serialize_text_small(&mut self, data: &BStr) -> Result {
    if data.len() > u8::MAX as usize {
      return Err(Error::new(ErrorKind::ArraySizeTooLarge));
    }

    self.serialize_u8(data.len() as u8)?;
    self.serialize_bytes(data.as_bytes())
  }
  pub fn serialize_text_large(&mut self, data: &BStr) -> Result {
    if data.len() > u16::MAX as usize {
      return Err(Error::new(ErrorKind::ArraySizeTooLarge));
    }

    self.serialize_u16(data.len() as u16)?;
    self.serialize_bytes(data.as_bytes())
  }

  pub fn serialize_accel(&mut self, v: Vector2<f32>) -> Result {
    ACCEL_SPEC.ser(self, v[0])?;
    ACCEL_SPEC.ser(self, v[1])
  }
  pub fn serialize_low_res_pos(&mut self, pos: Option<Vector2<f32>>) -> Result {
    let x = pos
      .map(|v| ((v[0] / 128.0) as i32 + 128) as u8)
      .unwrap_or(0);
    let y = pos
      .map(|v| ((v[1] / 128.0) as i32 + 128) as u8)
      .unwrap_or(0);

    x.serialize(self)?;
    y.serialize(self)
  }
  pub fn serialize_pos_f32(&mut self, pos: Vector2<f32>) -> Result {
    self.serialize_f32(pos[0])?;
    self.serialize_f32(pos[1])
  }
  pub fn serialize_pos(&mut self, pos: Vector2<f32>) -> Result {
    self.serialize_coordx(pos[0])?;
    self.serialize_coordy(pos[1])
  }
  pub fn serialize_pos24(&mut self, pos: Vector2<f32>) -> Result {
    self.serialize_coord24(pos[0])?;
    self.serialize_coord24(pos[1])
  }
  pub fn serialize_vel(&mut self, pos: Vector2<f32>) -> Result {
    self.serialize_speed(pos[0])?;
    self.serialize_speed(pos[1])
  }

  pub fn serialize_coord24(&mut self, v: f32) -> Result {
    let spec = COORD24_SPEC;
    self
      .serialize_u24(((v * spec.mult) as i32 + spec.shift).clamp(0, (u32::MAX >> 8) as i32) as u32)
  }
  pub fn serialize_coordx(&mut self, v: f32) -> Result {
    COORDX_SPEC.ser(self, v)
  }
  pub fn serialize_coordy(&mut self, v: f32) -> Result {
    COORDY_SPEC.ser(self, v)
  }
  pub fn serialize_energy(&mut self, v: f32) -> Result {
    ENERGY_SPEC.ser_u8(self, v)
  }
  pub fn serialize_regen(&mut self, v: f32) -> Result {
    REGEN_SPEC.ser(self, v)
  }
  pub fn serialize_rot(&mut self, v: f32) -> Result {
    ROTATION_SPEC.ser(self, v)
  }
  pub fn serialize_speed(&mut self, v: f32) -> Result {
    SPEED_SPEC.ser(self, v)
  }

  pub fn serialize_option_player(&mut self, p: Option<u16>) -> Result {
    p.unwrap_or(0).serialize(self)
  }
}

pub struct AirmashDeserializerV5<'de> {
  data: &'de [u8],
}

impl<'de> AirmashDeserializerV5<'de> {
  pub fn new(data: &'de [u8]) -> Self {
    Self { data }
  }

  pub fn remainder(&self) -> &'de [u8] {
    self.data
  }

  pub fn deserialize<T: DeserializeV5<'de>>(&mut self) -> Result<T> {
    T::deserialize(self)
  }

  pub fn deserialize_fixed<const N: usize>(&mut self) -> Result<[u8; N]> {
    use std::convert::TryInto;

    let slice: [u8; N] = self.data[..N]
      .try_into()
      .map_err(|_| Error::new(ErrorKind::EndOfBuffer))?;
    self.data = &self.data[N..];
    Ok(slice)
  }
  pub fn deserialize_bytes(&mut self, len: usize) -> Result<&'de [u8]> {
    if self.data.len() < len {
      return Err(Error::new(ErrorKind::EndOfBuffer));
    }

    let slice = &self.data[..len];
    self.data = &self.data[len..];
    Ok(slice)
  }

  pub fn deserialize_u8(&mut self) -> Result<u8> {
    if self.data.is_empty() {
      return Err(Error::new(ErrorKind::EndOfBuffer));
    }

    let value = self.data[0];
    self.data = &self.data[1..];
    Ok(value)
  }
  pub fn deserialize_u16(&mut self) -> Result<u16> {
    self.deserialize_fixed().map(u16::from_le_bytes)
  }
  pub fn deserialize_u32(&mut self) -> Result<u32> {
    self.deserialize_fixed().map(u32::from_le_bytes)
  }
  pub fn deserialize_u64(&mut self) -> Result<u64> {
    self.deserialize_fixed().map(u64::from_le_bytes)
  }
  pub fn deserialize_u128(&mut self) -> Result<u128> {
    self.deserialize_fixed().map(u128::from_le_bytes)
  }

  pub fn deserialize_i8(&mut self) -> Result<i8> {
    self.deserialize_u8().map(|x| x as i8)
  }
  pub fn deserialize_i16(&mut self) -> Result<i16> {
    self.deserialize_fixed().map(i16::from_le_bytes)
  }
  pub fn deserialize_i32(&mut self) -> Result<i32> {
    self.deserialize_fixed().map(i32::from_le_bytes)
  }
  pub fn deserialize_i64(&mut self) -> Result<i64> {
    self.deserialize_fixed().map(i64::from_le_bytes)
  }
  pub fn deserialize_i128(&mut self) -> Result<i128> {
    self.deserialize_fixed().map(i128::from_le_bytes)
  }

  pub fn deserialize_f32(&mut self) -> Result<f32> {
    self.deserialize_fixed().map(f32::from_le_bytes)
  }
  pub fn deserialize_f64(&mut self) -> Result<f64> {
    self.deserialize_fixed().map(f64::from_be_bytes)
  }

  pub fn deserialize_u24(&mut self) -> Result<u32> {
    let [a, b, c] = self.deserialize_fixed()?;
    Ok(u32::from_le_bytes([a, b, c, 0]))
  }
  pub fn deserialize_bool(&mut self) -> Result<bool> {
    Ok(self.deserialize_u8()? != 0)
  }

  pub fn deserialize_array_small<T>(&mut self) -> Result<Vec<T>>
  where
    T: DeserializeV5<'de>,
  {
    let len = self.deserialize_u8()? as usize;
    let mut data = Vec::with_capacity(len);

    for _ in 0..len {
      data.push(self.deserialize()?);
    }

    Ok(data)
  }
  pub fn deserialize_array_large<T>(&mut self) -> Result<Vec<T>>
  where
    T: DeserializeV5<'de>,
  {
    let len = self.deserialize_u16()? as usize;
    let mut data = Vec::with_capacity(len);

    for _ in 0..len {
      data.push(self.deserialize()?);
    }

    Ok(data)
  }

  pub fn deserialize_text_small(&mut self) -> Result<BString> {
    let len = self.deserialize_u8()? as usize;
    Ok(self.deserialize_bytes(len)?.into())
  }
  pub fn deserialize_text_large(&mut self) -> Result<BString> {
    let len = self.deserialize_u16()? as usize;
    Ok(self.deserialize_bytes(len)?.into())
  }

  pub fn deserialize_accel(&mut self) -> Result<Vector2<f32>> {
    Ok(Vector2::new(ACCEL_SPEC.de(self)?, ACCEL_SPEC.de(self)?))
  }
  pub fn deserialize_low_res_pos(&mut self) -> Result<Option<Vector2<f32>>> {
    let (x, y): (u8, u8) = self.deserialize()?;

    if x == 0 && y == 0 {
      return Ok(None);
    }

    Ok(Some(Vector2::new(
      ((x as i32 - 128) * 128) as f32,
      ((y as i32 - 128) * 128) as f32,
    )))
  }
  pub fn deserialize_pos_f32(&mut self) -> Result<Vector2<f32>> {
    Ok(Vector2::new(
      self.deserialize_f32()?,
      self.deserialize_f32()?,
    ))
  }
  pub fn deserialize_pos(&mut self) -> Result<Vector2<f32>> {
    Ok(Vector2::new(
      self.deserialize_coordx()?,
      self.deserialize_coordy()?,
    ))
  }
  pub fn deserialize_pos24(&mut self) -> Result<Vector2<f32>> {
    Ok(Vector2::new(
      self.deserialize_coord24()?,
      self.deserialize_coord24()?,
    ))
  }
  pub fn deserialize_vel(&mut self) -> Result<Vector2<f32>> {
    Ok(Vector2::new(
      self.deserialize_speed()?,
      self.deserialize_speed()?,
    ))
  }

  pub fn deserialize_coord24(&mut self) -> Result<f32> {
    let spec = COORD24_SPEC;
    Ok((((self.deserialize_u24()? as i32) - spec.shift) as f32) * (1.0 / spec.mult))
  }
  pub fn deserialize_coordx(&mut self) -> Result<f32> {
    COORDX_SPEC.de(self)
  }
  pub fn deserialize_coordy(&mut self) -> Result<f32> {
    COORDY_SPEC.de(self)
  }
  pub fn deserialize_energy(&mut self) -> Result<f32> {
    ENERGY_SPEC.de_u8(self)
  }
  pub fn deserialize_regen(&mut self) -> Result<f32> {
    REGEN_SPEC.de(self)
  }
  pub fn deserialize_rot(&mut self) -> Result<f32> {
    ROTATION_SPEC.de(self)
  }
  pub fn deserialize_speed(&mut self) -> Result<f32> {
    SPEED_SPEC.de(self)
  }

  pub fn deserialize_option_player(&mut self) -> Result<Option<u16>> {
    Ok(match self.deserialize_u16()? {
      0 => None,
      x => Some(x)
    })
  }
}

pub trait SerializeV5 {
  fn serialize(&self, ser: &mut AirmashSerializerV5) -> Result;
}

pub trait DeserializeV5<'de>: Sized {
  fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self>;
}

macro_rules! impl_builtin {
  ($ty:ty, $ser:ident, $de:ident) => {
    impl SerializeV5 for $ty {
      fn serialize<'ser>(&self, ser: &mut AirmashSerializerV5<'ser>) -> Result {
        ser.$ser(*self)
      }
    }

    impl<'de> DeserializeV5<'de> for $ty {
      fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
        de.$de()
      }
    }
  };
}

macro_rules! impl_tuple {
  () => { impl_tuple!(=); };
  ( $first:ident $(, $name:ident )* ) => {
    impl_tuple!(= $first $(, $name )*);
    impl_tuple!( $( $name ),* );
  };
  (= $( $name:ident ),* )=> {
    impl<$($name),*> SerializeV5 for ($( $name, )*)
    where $( $name: SerializeV5 ),*
    {
      #[allow(unused_variables, non_snake_case)]
      fn serialize(&self, ser: &mut AirmashSerializerV5) -> Result {
				let ($( $name, )*) = self;

        $( $name.serialize(ser)?; )*

        Ok(())
			}
    }

    impl<'de, $($name),*> DeserializeV5<'de> for ($( $name, )*)
    where $( $name: DeserializeV5<'de> ),*
    {
      #[allow(unused_variables, non_snake_case)]
      fn deserialize(de: &mut AirmashDeserializerV5<'de>) -> Result<Self> {
        Ok(( $( <$name>::deserialize(de)?, )* ))
			}
    }
  }
}

impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

impl_builtin!(u8, serialize_u8, deserialize_u8);
impl_builtin!(i8, serialize_i8, deserialize_i8);
impl_builtin!(u16, serialize_u16, deserialize_u16);
impl_builtin!(i16, serialize_i16, deserialize_i16);
impl_builtin!(u32, serialize_u32, deserialize_u32);
impl_builtin!(i32, serialize_i32, deserialize_i32);
impl_builtin!(u64, serialize_u64, deserialize_u64);
impl_builtin!(i64, serialize_i64, deserialize_i64);
impl_builtin!(u128, serialize_u128, deserialize_u128);
impl_builtin!(i128, serialize_i128, deserialize_i128);

impl_builtin!(f32, serialize_f32, deserialize_f32);
impl_builtin!(f64, serialize_f64, deserialize_f64);
impl_builtin!(bool, serialize_bool, deserialize_bool);
