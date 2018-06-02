
use airmash_protocol::FlagCode;

#[test]
fn flagcode_from_str_not_case_sensitive() {
	let caps = FlagCode::from_str("CA").unwrap();
	let lower = FlagCode::from_str("ca").unwrap();

	assert_eq!(caps, lower);
}

#[test]
fn flagcode_from_str_sanity() {
	assert_eq!(FlagCode::from_str("us").unwrap(), FlagCode::UnitedStates);
}
