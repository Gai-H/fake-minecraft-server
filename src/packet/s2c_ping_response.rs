use std::fmt::Debug;
use std::io::Write;
use std::net::TcpStream;
use crate::datatype::{long, varint};
use crate::packet::{ClientBoundPacketBody, PacketBody};
use crate::session::Session;

#[derive(Debug)]
pub struct S2CPingResponsePacket {
    payload: long::Long
}

impl S2CPingResponsePacket {
    pub const PACKET_ID: i32 = 0x01;

    pub fn new(payload: long::Long) -> S2CPingResponsePacket {
        S2CPingResponsePacket {
            payload
        }
    }
}

impl PacketBody for S2CPingResponsePacket {
    fn update_session(&self, _: &mut Session) {
    }

    fn handle(&self, _: &mut Session, _: &mut TcpStream) -> Result<(), std::string::String> {
        Ok(())
    }
}

impl ClientBoundPacketBody for S2CPingResponsePacket {
    fn write_to_stream(&self, stream: &mut impl Write) -> Result<(), std::string::String> {
        let packet_id_bytes: Vec<u8> = varint::VarInt::from(S2CPingResponsePacket::PACKET_ID).into();

        let value_bytes: Vec<u8> = self.payload.clone().into();

        let packet_length: i32 = packet_id_bytes.len() as i32 + value_bytes.len() as i32;
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length).into();

        let bytes: Vec<u8> = [&packet_length_bytes[..], &packet_id_bytes[..], &value_bytes[..]].concat();

        if stream.write_all(&bytes).is_err() {
            return Err("Could not write packet to stream.".to_string());
        }

        if stream.flush().is_err() {
            return Err("Could not flush stream.".to_string());
        }

        Ok(())
    }
}