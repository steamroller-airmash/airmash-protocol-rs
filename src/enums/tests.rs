#[cfg(feature = "serde")]
mod serde {
  use crate::*;

  macro_rules! roundtrip_planetype_test {
    ($test:ident => $plane:ident) => {
      #[test]
      fn $test() {
        let expected = PlaneType::$plane;
        let serialized = serde_json::to_string(&PlaneType::$plane)
          .expect(concat!("Failed to serialize ", stringify!($plane)));
        eprintln!("serialized = {}", serialized);

        let result = serde_json::from_str(&serialized)
          .expect(concat!("Failed to deserialize ", stringify!($plane)));

        assert_eq!(expected, result, "serialized = {}", serialized);
      }
    };
  }

  roundtrip_planetype_test!(roundtrip_predator => Predator);
  roundtrip_planetype_test!(roundtrip_tornado => Tornado);
  roundtrip_planetype_test!(roundtrip_mohawk => Mohawk);
  roundtrip_planetype_test!(roundtrip_goliath => Goliath);
  roundtrip_planetype_test!(roundtrip_prowler => Prowler);

  #[test]
  fn planetype_deserialize_test() {
    use self::PlaneType::*;

    fn de(data: &str) -> PlaneType {
      serde_json::from_str(data) //
        .expect(&format!("Failed to deserialize planetype from {}", data))
    }

    // raw numerical values should work
    assert_eq!(Predator, de("1"));

    // same with all casings of the word
    assert_eq!(Predator, de("\"Predator\""));
    assert_eq!(Predator, de("\"predator\""));
    assert_eq!(Predator, de("\"PrEdAtOr\""));

    assert_eq!(Unknown(0), de("0"));
    assert_eq!(Unknown(255), de("255"));
  }
}
