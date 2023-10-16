use super::{varint, DatatypeError};
use std::io::Read;

#[derive(Debug, PartialEq, Clone)]
pub struct String {
    pub value: std::string::String,
}

impl String {
    pub const MAX_LENGTH: i32 = 32767;
}

impl From<&str> for String {
    fn from(v: &str) -> Self {
        String {
            value: v.to_string(),
        }
    }
}

impl From<std::string::String> for String {
    fn from(v: std::string::String) -> Self {
        String { value: v }
    }
}

impl Into<Vec<u8>> for String {
    fn into(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        let mut bytes = self.value.into_bytes();
        let length: varint::VarInt = (bytes.len() as i32).into();
        res.append(&mut length.into());
        res.append(&mut bytes);
        res
    }
}

pub fn read_from_stream(stream: &mut impl Read) -> Result<String, DatatypeError> {
    let length = varint::read_from_stream(stream).unwrap();
    if length.value > String::MAX_LENGTH {
        return Err(DatatypeError::TooLongStringError);
    }

    let mut bytes: Vec<u8> = Vec::new();
    let mut byte = [0; 1];
    for _ in 0..length.value {
        match stream.read_exact(&mut byte[..]) {
            Ok(_) => bytes.push(byte[0]),
            Err(_) => return Err(DatatypeError::ReadError),
        }
    }

    return match std::string::String::from_utf8(bytes) {
        Ok(s) => Ok(String { value: s }),
        Err(_) => Err(DatatypeError::ConvertError),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_read_from_stream() {
        let mut bytes: VecDeque<u8> = VecDeque::from([
            0x1c, 0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3,
            0x81, 0xaf, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0xf0, 0x9f, 0x8c, 0x9f, 0xc2,
            0xa5,
        ]);
        assert_eq!(
            read_from_stream(&mut bytes),
            Ok(String::from("こんにちは, World🌟¥"))
        );
    }

    #[test]
    fn test_into() {
        let s = String::from("こんにちは, World🌟¥");
        let s_into: Vec<u8> = s.into();
        let bytes: Vec<u8> = Vec::from([
            0x1c, 0xe3, 0x81, 0x93, 0xe3, 0x82, 0x93, 0xe3, 0x81, 0xab, 0xe3, 0x81, 0xa1, 0xe3,
            0x81, 0xaf, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0xf0, 0x9f, 0x8c, 0x9f, 0xc2,
            0xa5,
        ]);
        assert_eq!(s_into, bytes);
    }
}
