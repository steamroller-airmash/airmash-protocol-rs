

#[cfg(test)]
mod test {
    use field::*;
    use ser::Serializer;
    use de::Deserializer;

    #[test]
    fn text_serialize() {
        let mut ser = Serializer{ output: vec![] };
        let val = "test string";

        text::serialize(val, &mut ser);

        let bytes = [
            11,
            't' as u8,
            'e' as u8,
            's' as u8,
            't' as u8,
            ' ' as u8,
            's' as u8,
            't' as u8,
            'r' as u8,
            'i' as u8,
            'n' as u8,
            'g' as u8
        ];

        for i in 0..bytes.len() {
            println!("{}", i);
            assert_eq!(ser.output[i], bytes[i]);
        }
    }

    #[test]
    fn text_deserialize() {
        let bytes = [
            8,
            'h' as u8,
            'i' as u8,
            ' ' as u8,
            't' as u8,
            'h' as u8,
            'e' as u8,
            'r' as u8,
            'e' as u8
        ];

        let mut de = Deserializer{ bytes: &bytes };
        let val = text::deserialize(&mut de).unwrap();

        assert_eq!("hi there", &val);
    }
}
