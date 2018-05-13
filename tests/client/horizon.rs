
use airmash_protocol::{from_bytes, to_bytes};
use airmash_protocol::client::Horizon;

#[test]
fn test_serialize() {
    let bytes = to_bytes(&Horizon { 
        horizon_x: 0xFFFF,
        horizon_y: 0xFFFF
    }).unwrap();

    assert_eq!(bytes.len(), 4);

    for x in bytes {
        assert_eq!(x, 0xFF);
    }
}

#[test]
fn test_deserialize() {
    let bytes = [
        0xCD,
        0xAB,
        0x34,
        0x12,
    ];

    let result = from_bytes::<Horizon>(&bytes).unwrap();

    assert_eq!(result.horizon_x, 0xABCD);
    assert_eq!(result.horizon_y, 0x1234);
}
