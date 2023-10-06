use std::io::Read;
use std::net::TcpStream;
use crate::packet::{ClientBoundPacketBody, PacketBody, ServerBoundPacketBody};
use crate::packet::s2c_status_response::S2CStatusResponsePacket;
use crate::session::Session;

#[derive(Debug)]
pub struct C2SStatusRequestPacket {
}

impl C2SStatusRequestPacket {
    pub const PACKET_ID: i32 = 0x00;

    const NEXT_PACKET_IDS: [i32; 1] = [0x01]; // Status Ping
}

impl PacketBody for C2SStatusRequestPacket {
    fn update_session(&self, session: &mut Session) {
        session.next_packet_ids = &C2SStatusRequestPacket::NEXT_PACKET_IDS;
    }

    fn handle(&self, _: &mut Session, stream: &mut TcpStream) -> Result<(), String> {
        let response_packet = S2CStatusResponsePacket::new();
        response_packet.write_to_stream(stream)
    }
}

impl ServerBoundPacketBody for C2SStatusRequestPacket {
    fn read_from_stream(_: &mut impl Read) -> Result<Box<dyn PacketBody>, std::string::String> {
        Ok(Box::new(C2SStatusRequestPacket {
        }))
    }
}