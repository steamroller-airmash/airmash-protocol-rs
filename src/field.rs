use serde::*;

pub type SerResult = Result<()>;

pub mod textbig {
    use field::*;

    pub fn serialize(val: &str, ser: &mut Serializer) -> SerResult {
        let bytes = val.as_bytes();

        // This is a stopgap for now, returning an error
        // would be a better idea
        assert!(bytes.len() <= 0xFFFF);

        ser.serialize_u16(bytes.len() as u16)?;
        ser.serialize_bytes(bytes)
    }
    pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<String> {
        let len = de.deserialize_u16()?;
        Ok(de.deserialize_str(len as usize)?.to_string())
    }
}

pub mod text {
    use field::*;

    pub fn serialize(val: &str, ser: &mut Serializer) -> SerResult {
        let bytes = val.as_bytes();

        // Ensure that the length of the string is valid
        assert!(bytes.len() <= 0xFF);

        ser.serialize_u8(bytes.len() as u8)?;
        ser.serialize_bytes(bytes)
    }
    pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<String> {
        let len = de.deserialize_u8()?;
        Ok(de.deserialize_str(len as usize)?.to_string())
    }
}

pub mod array {
    use field::*;
    use std::vec::Vec;

    pub fn serialize<T>(arr: &Vec<T>, ser: &mut Serializer) -> SerResult
    where
        T: Serialize,
    {
        // Require that array length is valid
        assert!(arr.len() <= 0xFFFF);

        let s = ser.serialize_u16(arr.len() as u16)?;

        for val in arr {
            val.serialize(ser)?;
        }

        Ok(s)
    }
    pub fn deserialize<'de, T>(de: &mut Deserializer<'de>) -> Result<Vec<T>>
    where
        T: Deserialize<'de>,
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
    use field::*;

    pub fn serialize<T>(arr: &[T], ser: &mut Serializer) -> SerResult
    where
        T: Serialize,
    {
        assert!(arr.len() <= 0xFF);

        let s = ser.serialize_u8(arr.len() as u8)?;

        for val in arr {
            val.serialize(ser)?;
        }

        Ok(s)
    }
    pub fn deserialize<'de, T>(de: &mut Deserializer<'de>) -> Result<Vec<T>>
    where
        T: Deserialize<'de>,
    {
        let len = de.deserialize_u8()?;
        let mut vec = vec![];

        for _ in 0..len {
            vec.push(T::deserialize(de)?);
        }

        Ok(vec)
    }
}

pub mod rotation {
    use field::*;

    const MULT: f32 = 6553.6;

    pub fn serialize(val: &f32, ser: &mut Serializer) -> SerResult {
        ser.serialize_u16((*val * MULT) as u16)
    }
    pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<f32> {
        Ok((de.deserialize_u16()? as f32) / MULT)
    }
}

pub mod healthnergy {
    use field::*;

    const MULT: f32 = 255.0;

    pub fn serialize(val: &f32, ser: &mut Serializer) -> SerResult {
        ser.serialize_u16((*val * MULT) as u16)
    }
    pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<f32> {
        Ok((de.deserialize_u16()? as f32) / MULT)
    }
}

pub mod uint24 {
    use field::*;

    pub fn serialize(val: u32, ser: &mut Serializer) -> SerResult {
        ser.serialize_u16((val >> 8) as u16)?;
        ser.serialize_u8(val as u8)
    }
    pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<u32> {
        let hi = de.deserialize_u16()?;
        let lo = de.deserialize_u8()?;

        Ok(((hi << 8) as u32) | (lo as u32))
    }
}

pub mod coord24 {
    use field::*;

    // Note: This assumes that f32 has enough precision,
    //       the client uses f64 as it is written in js

    const SHIFT: i32 = 8388608;
    const MULT: f32 = 512.0;

    pub fn serialize(val: &f32, ser: &mut Serializer) -> SerResult  {
        uint24::serialize(((*val * MULT) as i32 + SHIFT) as u32, ser)
    }
    pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<f32> {
        Ok((((uint24::deserialize(de)? as i32) - SHIFT) as f32) / MULT)
    }
}

macro_rules! shift_mult_decode {
    ($name:ident, $shift:expr, $mult:expr) => {
        pub mod $name {
            use field::*;

            // Note: This assumes that f32 has enough precision,
            //       the client uses f64 as it is written in js

            const SHIFT: i32 = $shift;
            const MULT: f32 = $mult;

            pub fn serialize(val: &f32, ser: &mut Serializer) -> SerResult {
                ser.serialize_u16(((*val * MULT) as i32 + SHIFT) as u16)
            }
            pub fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<f32> {
                Ok((((de.deserialize_u16()? as i32) - SHIFT) as f32) / MULT)
            }
        }
    };
}

shift_mult_decode!(coordy, 32768, 4.0);
shift_mult_decode!(coordx, 32768, 2.0);
shift_mult_decode!(regen, 32768, 1.0e6);
shift_mult_decode!(accel, 32768, 32768.0);
shift_mult_decode!(speed, 32768, 1638.4);
