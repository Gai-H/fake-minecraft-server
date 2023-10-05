use std::io::Read;
use crate::datatype::varint;

#[derive(Debug, PartialEq)]
pub struct String {
    pub value: std::string::String
}

impl String {
    pub const MAX_LENGTH: i32 = 32767;
}

pub fn read_from_stream(stream: &mut impl Read) -> Result<String, std::string::String> {
    let length = varint::read_from_stream(stream).unwrap();
    if length.value > String::MAX_LENGTH {
        return Err("String length is too long.".to_string());
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut byte = [0; 1];
    for _ in 0..length.value {
        if let Ok(_) = stream.read_exact(&mut byte[..]) {
            bytes.push(byte[0]);
        } else {
            return Err("Could not read bytes from stream.".to_string());
        }
    }

    return if let Ok(s) = std::string::String::from_utf8(bytes) {
        Ok(String { value: s })
    } else {
        Err("Could not convert bytes to string.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::*;

    #[test]
    fn test_read_from_stream() {
        let mut bytes: VecDeque<u8> = VecDeque::from([0x1c, 0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3, 0x81, 0xaf, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0xf0, 0x9f, 0x8c, 0x9f, 0xc2, 0xa5]);
        assert_eq!(read_from_stream(&mut bytes), Ok(String { value: "こんにちは, World🌟¥".to_string() }));
    }
}