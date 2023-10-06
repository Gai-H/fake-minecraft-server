use std::io::Read;
use std::net::TcpStream;
use crate::datatype::long;
use crate::packet::{ClientBoundPacketBody, PacketBody, ServerBoundPacketBody};
use crate::packet::s2c_ping_response::S2CPingResponsePacket;
use crate::session::Session;

#[derive(Debug)]
pub struct C2SPingRequestPacket {
    pub payload: long::Long
}

impl C2SPingRequestPacket {
    pub const PACKET_ID: i32 = 0x01;

    const NEXT_PACKET_IDS: [i32; 0] = []; // terminate connection
}

impl PacketBody for C2SPingRequestPacket {
    fn update_session(&self, session: &mut Session) {
        session.next_packet_ids = &C2SPingRequestPacket::NEXT_PACKET_IDS;
    }

    fn handle(&self, _: &mut Session, stream: &mut TcpStream) -> Result<(), std::string::String> {
        let response_packet = S2CPingResponsePacket::new(self.payload.clone());
        response_packet.write_to_stream(stream)
    }
}

impl ServerBoundPacketBody for C2SPingRequestPacket {
    fn read_from_stream(stream: &mut impl Read) -> Result<Box<dyn PacketBody>, std::string::String> {
        let value = long::read_from_stream(stream).unwrap();

        Ok(Box::new(C2SPingRequestPacket {
            payload: value
        }))
    }
}