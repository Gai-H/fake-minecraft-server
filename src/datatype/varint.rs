use std::io::Read;
use std::result::Result;

#[derive(Debug)]
pub struct VarInt {
    pub value: i32
}

impl VarInt {
    const SEGMENT_BITS: u8 = 0b0111_1111;
    const CONTINUE_BIT: u8 = 0b1000_0000;
}

impl From<&[u8]> for VarInt {
    fn from(v: &[u8]) -> Self {
        let mut res: i32 = 0;
        let mut shift: u32 = 0;
        for &byte in v {
            res |= ((byte & Self::SEGMENT_BITS) as i32) << shift;
            shift += 7;
            if byte & Self::CONTINUE_BIT == 0 {
                break;
            }
        }
        VarInt { value: res }
    }
}

impl From<i32> for VarInt {
    fn from(v: i32) -> Self {
        VarInt { value: v }
    }
}

pub fn read_from_stream(stream: &mut impl Read) -> Result<VarInt, std::string::String> {
    let mut varint_bytes: Vec<u8> = Vec::new();
    let mut byte = [0; 1];
    loop {
        if let Ok(_) = stream.read_exact(&mut byte[..]) {
            varint_bytes.push(byte[0]);
        } else {
            return Err("Could not read bytes from stream.".to_string());
        }

        if byte[0] & VarInt::CONTINUE_BIT == 0 {
            break;
        }
    }
    Ok(VarInt::from(&varint_bytes[..]))
}