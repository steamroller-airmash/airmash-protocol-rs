

#[cfg(test)]
mod test {
    use field::*;
    use serde::{Serializer, Deserializer};
    use std::vec::Vec;

    fn text_to_vec(val: &str) -> Vec<u8> {
        let mut bytes = vec![ val.len() as u8 ];

        for byte in val.bytes() {
            bytes.push(byte);
        }

        bytes
    }

    #[test]
    fn text_serialize() {
        let mut ser = Serializer{ output: vec![] };
        let val = "test string";
        let bytes = text_to_vec(val);

        text::serialize(val, &mut ser).unwrap();

        for i in 0..bytes.len() {
            println!("{}", i);
            assert_eq!(ser.output[i], bytes[i]);
        }
    }

    #[test]
    fn text_deserialize() {
        let text = "hi there";
        let bytes = text_to_vec(text);

        let mut de = Deserializer{ bytes: &bytes[..] };
        let val = text::deserialize(&mut de).unwrap();

        assert_eq!(text, &val);
    }
}
