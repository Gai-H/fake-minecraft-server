use std::io::Read;
use crate::datatype::DatatypeError;

#[derive(Debug, PartialEq, Clone)]
pub struct Long {
    pub value: i64
}

impl From<&[u8; 8]> for Long {
    fn from(v: &[u8; 8]) -> Self {
        Long { value: i64::from_be_bytes(*v) }
    }
}

impl From<i64> for Long {
    fn from(v: i64) -> Self {
        Long { value: v }
    }
}

impl Into<Vec<u8>> for Long {
    fn into(self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

pub fn read_from_stream(stream: &mut impl Read) -> Result<Long, DatatypeError> {
    let mut bytes: [u8; 8] = [0; 8];
    if let Ok(_) = stream.read_exact(&mut bytes[..]) {
        Ok(Long::from(&bytes))
    } else {
        Err(DatatypeError::ReadError)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::*;

    #[test]
    fn test_from_u8_array() {
        let bytes: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff];
        let long = Long::from(&bytes);
        assert_eq!(long.value, 2147483647);
    }

    #[test]
    fn test_into() {
        let long = Long::from(2147483647);
        let bytes: Vec<u8> = long.into();
        assert_eq!(bytes, [0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn test_read_from_stream() {
        let mut bytes: VecDeque<u8> = VecDeque::from([0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0xff, 0xff]);
        assert_eq!(read_from_stream(&mut bytes), Ok(Long::from(2147483647)));
    }
}