use std::io::Write;
use crate::datatype::{string, varint};
use crate::packet::{ClientBoundPacketBody, PacketBody};

#[derive(Debug)]
pub struct S2CStatusResponsePacket {
}

impl S2CStatusResponsePacket {
    pub const PACKET_ID: i32 = 0x00;

    pub const RESPONSE_JSON: &'static str = "{\"version\":{\"name\":\"1.20.2\",\"protocol\":764},\"enforcesSecureChat\":true,\"description\":{\"text\":\"A Fake Minecraft Server\"},\"players\":{\"max\":42,\"online\":0}}";

    pub fn new() -> S2CStatusResponsePacket {
        S2CStatusResponsePacket {
        }
    }
}

impl PacketBody for S2CStatusResponsePacket {
    fn update_session(&self, _: &mut crate::session::Session) {
    }
}

impl ClientBoundPacketBody for S2CStatusResponsePacket {
    fn write_to_stream(&self, stream: &mut impl Write) -> Result<(), String> {
        let packet_id_bytes: Vec<u8> = varint::VarInt::from(S2CStatusResponsePacket::PACKET_ID).into();

        let response_json_bytes: Vec<u8> = string::String::from(S2CStatusResponsePacket::RESPONSE_JSON).into();

        let packet_length: i32 = packet_id_bytes.len() as i32 + response_json_bytes.len() as i32;
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length).into();

        let bytes: Vec<u8> = [&packet_length_bytes[..], &packet_id_bytes[..], &response_json_bytes[..]].concat();

        if stream.write_all(&bytes).is_err() {
            return Err("Could not write packet to stream.".to_string());
        }

        if stream.flush().is_err() {
            return Err("Could not flush stream.".to_string());
        }

        Ok(())
    }
}