use crate::packet::{
    s2c_status_response, ClientBoundPacketBody, PacketBody, Result, ServerBoundPacketBody,
};
use crate::session::Session;
use std::io::Read;
use std::net::TcpStream;

#[derive(Debug)]
pub struct C2SStatusRequestPacket {}

impl C2SStatusRequestPacket {
    pub const PACKET_ID: i32 = 0x00;

    const NEXT_PACKET_IDS: [i32; 1] = [0x01]; // Status Ping
}

impl PacketBody for C2SStatusRequestPacket {
    fn update_session(&self, session: &mut Session) {
        session.next_packet_ids = &C2SStatusRequestPacket::NEXT_PACKET_IDS;
    }
}

impl ServerBoundPacketBody for C2SStatusRequestPacket {
    fn read_from_stream(
        _: &mut Session,
        _: &mut impl Read,
    ) -> Result<Box<dyn ServerBoundPacketBody>> {
        Ok(Box::new(C2SStatusRequestPacket {}))
    }

    fn respond(&self, session: &mut Session, stream: &mut TcpStream) -> Result<()> {
        let response_packet = s2c_status_response::S2CStatusResponsePacket::new();
        response_packet.write_to_stream(session, stream).into()
    }
}
