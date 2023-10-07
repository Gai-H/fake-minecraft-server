use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct UUID {
    pub value: u128
}

impl From<&[u8; 16]> for UUID {
    fn from(bytes: &[u8; 16]) -> Self {
        UUID { value: u128::from_be_bytes(*bytes) }
    }
}

impl From<u128> for UUID {
    fn from(value: u128) -> Self {
        UUID { value }
    }
}

pub fn read_from_stream(stream: &mut impl Read) -> Result<UUID, &'static str> {
    let mut bytes: [u8; 16] = [0; 16];
    if let Ok(_) = stream.read_exact(&mut bytes[..]) {
        Ok(UUID::from(&bytes))
    } else {
        Err("Could not read bytes from stream.")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::datatype::uuid;
    use crate::datatype::uuid::UUID;

    #[test]
    fn test_read_from_stream() {
        let mut bytes: VecDeque<u8> = VecDeque::from([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10]);
        assert_eq!(uuid::read_from_stream(&mut bytes), Ok(UUID::from(0x0123456789abcdeffedcba9876543210)));
    }
}