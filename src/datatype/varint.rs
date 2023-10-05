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

impl Into<Vec<u8>> for VarInt {
    fn into(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        let mut value: u32 = if self.value < 0 {
            u32::from_be_bytes(self.value.to_be_bytes())
        } else {
            self.value as u32
        };

        loop {
            if ((value as u8) & !Self::SEGMENT_BITS) == 0 {
                res.push(value as u8);
                break;
            } else {
                res.push(((value as u8) & Self::SEGMENT_BITS) | Self::CONTINUE_BIT);
                value >>= 7;
            }
        }
        res
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

#[cfg(test)]
mod test {
    use crate::datatype::varint::VarInt;

    #[test]
    fn test_into_positive() {
        let vi_positive: VarInt = VarInt::from(150);
        let vi_positive_into: Vec<u8> = vi_positive.into();
        assert_eq!(vec![150, 1], vi_positive_into);
    }

    #[test]
    fn test_into_negative() {
        let vi_negative: VarInt = VarInt::from(-1);
        let vi_negative_into: Vec<u8> = vi_negative.into();
        assert_eq!(vec![255, 255, 255, 255, 15], vi_negative_into);
    }

    #[test]
    fn test_into_zero() {
        let vi_zero: VarInt = VarInt::from(0);
        let vi_zero_into: Vec<u8> = vi_zero.into();
        assert_eq!(vec![0], vi_zero_into);
    }
}