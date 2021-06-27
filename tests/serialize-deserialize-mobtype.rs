#![cfg(feature = "serde")]

use airmash_protocol::MobType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TestStruct {
  mob: MobType,
}
