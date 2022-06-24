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

  fn de<'de, T: serde::Deserialize<'de>>(data: &'de str) -> T {
    serde_json::from_str(data) //
      .expect(&format!("Failed to deserialize from `{}`", data))
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
        .expect(&format!("Failed to deserialize planetype from `{}`", data))
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

  #[test]
  fn mobtype_deserialize_test() {
    use self::MobType::*;

    // raw numerical values should work
    assert_eq!(PredatorMissile, de("1"));

    // same with all casings
    assert_eq!(PredatorMissile, de(r#""PredatorMissile""#));
    assert_eq!(PredatorMissile, de(r#""PREDATORMISSILE""#));
    assert_eq!(PredatorMissile, de(r#""predatormissile""#));
    assert_eq!(PredatorMissile, de(r#""pReDaToRmIsSiLe""#));

    // we also support kebab-case, snake_case, and SCREAMING_SNAKE_CASE
    assert_eq!(PredatorMissile, de(r#""predator-missile""#));
    assert_eq!(PredatorMissile, de(r#""predator_missile""#));
    assert_eq!(PredatorMissile, de(r#""PREDATOR_MISSILE""#));
  }

  #[test]
  fn firewall_status_test() {
    use self::FirewallStatus::*;

    // the catchall case should work as expectd
    assert_eq!(Removed, de("0"));
    assert_eq!(Present, de("2"));
    assert_eq!(Present, de("255"));
  }

  #[test]
  #[should_panic]
  fn firewall_status_out_of_bounds() {
    de::<FirewallStatus>("256");
  }
}
