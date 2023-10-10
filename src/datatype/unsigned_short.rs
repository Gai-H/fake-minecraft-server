use std::io::Read;
use crate::datatype::DatatypeError;

#[derive(Debug)]
pub struct UnsignedShort {
    pub value: u16
}

pub fn read_from_stream(stream: &mut impl Read) -> Result<UnsignedShort, DatatypeError> {
    let mut bytes: [u8; 2] = [0; 2];
    return if let Ok(_) = stream.read_exact(&mut bytes[..]) {
        Ok(UnsignedShort { value: ((bytes[0] as u16) << 8) | (bytes[1] as u16) })
    } else {
        Err(DatatypeError::ReadError)
    }
}