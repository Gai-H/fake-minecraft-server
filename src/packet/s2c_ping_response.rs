use crate::datatype::{long, varint};
use crate::packet;
use crate::packet::{ClientBoundPacketBody, PacketBody, PacketError};
use crate::session::Session;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug)]
pub struct S2CPingResponsePacket {
    payload: long::Long,
}

impl S2CPingResponsePacket {
    pub const PACKET_ID: i32 = 0x01;

    pub fn new(payload: long::Long) -> S2CPingResponsePacket {
        S2CPingResponsePacket { payload }
    }
}

impl PacketBody for S2CPingResponsePacket {
    fn update_session(&self, _: &mut Session) {}
}

impl ClientBoundPacketBody for S2CPingResponsePacket {
    fn write_to_stream(&self, _: &mut Session, stream: &mut impl Write) -> packet::Result<()> {
        let packet_id_bytes: Vec<u8> =
            varint::VarInt::from(S2CPingResponsePacket::PACKET_ID).into();

        let value_bytes: Vec<u8> = self.payload.clone().into();

        let packet_length: usize = packet_id_bytes.len() + value_bytes.len();
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length as i32).into();

        let bytes: Vec<u8> = [
            &packet_length_bytes[..],
            &packet_id_bytes[..],
            &value_bytes[..],
        ]
        .concat();

        if stream.write_all(&bytes).is_err() {
            return Err(PacketError::WriteError.into());
        }

        if stream.flush().is_err() {
            return Err(PacketError::FlushError.into());
        }

        Ok(())
    }
}
