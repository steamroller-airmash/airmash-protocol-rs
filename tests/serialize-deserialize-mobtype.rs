
#![cfg(feature = "serde")]

use serde::{Serialize, Deserialize};
use airmash_protocol::MobType;

#[derive(Serialize, Deserialize)]
struct TestStruct {
  mob: MobType
}
