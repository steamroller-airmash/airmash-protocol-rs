
use serde_am::*;
use error::Error;
use std::result;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub enum PlaneType {
    Predator,
    Tornado,
    Mohawk,
    Goliath,
    Prowler
}

const PREDATOR: u8 = 1;
const GOLIATH: u8 = 2;
const MOHAWK: u8 = 3;
const TORNADO: u8 = 4;
const PROWLER: u8 = 5;

impl PlaneType {
    pub fn to_u8(self) -> u8 {
        match self {
            PlaneType::Predator => PREDATOR,
            PlaneType::Tornado =>  TORNADO,
            PlaneType::Mohawk =>   MOHAWK,
            PlaneType::Goliath =>  GOLIATH,
            PlaneType::Prowler =>  PROWLER
        }
    }

    pub fn from_u8(v: u8) -> result::Result<Self, u8> {
        Ok(match v {
            PREDATOR => PlaneType::Predator,
            TORNADO  => PlaneType::Tornado,
            MOHAWK   => PlaneType::Mohawk,
            GOLIATH  => PlaneType::Goliath,
            PROWLER  => PlaneType::Prowler,
            _ => return Err(v)
        })
    }
}

impl Serialize for PlaneType {
    fn serialize(&self, ser: &mut Serializer) -> Result<()> {
        self.to_u8().serialize(ser)
    }
}
impl<'de> Deserialize<'de> for PlaneType {
    fn deserialize(de: &mut Deserializer<'de>) -> Result<Self> {
        match Self::from_u8(de.deserialize_u8()?) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::InvalidPlaneType(e))
        }
    }
}
