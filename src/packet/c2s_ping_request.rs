use super::datatype::long;
use crate::packet;
use crate::packet::s2c_ping_response::S2CPingResponsePacket;
use crate::packet::{ClientBoundPacketBody, PacketBody, ServerBoundPacketBody};
use crate::session::Session;
use std::io::Read;
use std::net::TcpStream;

#[derive(Debug)]
pub struct C2SPingRequestPacket {
    pub payload: long::Long,
}

impl C2SPingRequestPacket {
    pub const PACKET_ID: i32 = 0x01;

    const NEXT_PACKET_IDS: [i32; 0] = []; // terminate connection
}

impl PacketBody for C2SPingRequestPacket {
    fn update_session(&self, session: &mut Session) {
        session.next_packet_ids = &C2SPingRequestPacket::NEXT_PACKET_IDS;
    }
}

impl ServerBoundPacketBody for C2SPingRequestPacket {
    fn read_from_stream(
        _: &mut Session,
        stream: &mut impl Read,
    ) -> packet::Result<Box<dyn ServerBoundPacketBody>> {
        let value = long::read_from_stream(stream)?;

        Ok(Box::new(C2SPingRequestPacket { payload: value }))
    }

    fn respond(&self, session: &mut Session, stream: &mut TcpStream) -> packet::Result<()> {
        let response_packet = S2CPingResponsePacket::new(self.payload.clone());
        response_packet.write_to_stream(session, stream).into()
    }
}
