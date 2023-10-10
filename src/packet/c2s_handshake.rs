use std::fmt::Debug;
use std::io::Read;
use std::net::TcpStream;
use crate::session::{Session, SessionState};
use crate::datatype::{string, varint, unsigned_short};
use crate::packet::{PacketBody, ServerBoundPacketBody};

#[derive(Debug)]
pub struct C2SHandshakePacket {
    protocol_version: varint::VarInt,
    server_address: string::String,
    server_port: unsigned_short::UnsignedShort,
    next_state: varint::VarInt
}

impl C2SHandshakePacket {
    pub const PACKET_ID: i32 = 0x00;

    const NEXT_PACKET_IDS_STATUS: [i32; 1] = [0x00]; // Status Request

    const NEXT_PACKET_IDS_LOGIN: [i32; 1] = [0x00]; // Login Start
}

impl PacketBody for C2SHandshakePacket {
    fn update_session(&self, session: &mut Session) {
        session.protocol_version = Some(self.protocol_version.value);
        session.server_address = Some(self.server_address.value.clone());
        session.server_port = Some(self.server_port.value);
        match self.next_state.value {
            1 => {
                session.state = SessionState::STATUS;
                session.next_packet_ids = &C2SHandshakePacket::NEXT_PACKET_IDS_STATUS;
            }
            2 => {
                session.state = SessionState::LOGIN;
                session.next_packet_ids = &C2SHandshakePacket::NEXT_PACKET_IDS_LOGIN;
            }
            _ => unreachable!()
        }
    }
}

impl ServerBoundPacketBody for C2SHandshakePacket {
    fn read_from_stream(_: &mut Session, stream: &mut impl Read) -> Result<Box<dyn ServerBoundPacketBody>, std::string::String> {
        let protocol_version = varint::read_from_stream(stream).unwrap();

        let server_address = string::read_from_stream(stream).unwrap();

        let server_port = unsigned_short::read_from_stream(stream).unwrap();

        let next_state = varint::read_from_stream(stream).unwrap();
        if next_state.value != 1 && next_state.value != 2 {
            return Err(format!("Invalid next state for C2SHandshakePacket: {}", next_state.value));
        }

        Ok(Box::new(C2SHandshakePacket {
            protocol_version,
            server_address,
            server_port,
            next_state
        }))
    }
    fn respond(&self, _: &mut Session, _: &mut TcpStream) -> Result<(), std::string::String> {
        Ok(())
    }
}
