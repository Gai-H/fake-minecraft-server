use std::fmt::Debug;
use std::io::Read;
use std::net::TcpStream;
use crate::datatype::varint;
use crate::session::{Session, SessionState};

pub mod c2s_handshake;

#[derive(Debug)]
pub struct PacketHeader {
    pub length: i32,
    pub id: i32
}

pub fn read_packet_header_from_stream(stream: &mut TcpStream) -> Result<PacketHeader, String> {
    let packet_length = varint::read_from_stream(stream).unwrap();
    let packet_id = varint::read_from_stream(stream).unwrap();

    Ok(PacketHeader {
        length: packet_length.value,
        id: packet_id.value
    })
}

pub trait PacketBody: Debug {
    fn update_session(&self, session: &mut Session);
    fn handle(&self, session: &mut Session);
}

pub trait ServerBoundPacket {
    fn read_from_stream(stream: &mut impl Read) -> Result<Box<dyn PacketBody>, std::string::String>;
}

pub fn read_packet_body_from_stream(stream: &mut TcpStream, session: &Session, header: &PacketHeader) -> Result<Box<dyn PacketBody>, std::string::String> {
    return match session.state {
        SessionState::HANDSHAKING => {
            match header.id {
                c2s_handshake::C2SHandshakePacket::PACKET_ID => {
                    let packet = c2s_handshake::C2SHandshakePacket::read_from_stream(stream).unwrap();
                    Ok(packet)
                },
                _ => {
                    Err(format!("Invalid packet id {}", header.id))
                }
            }
        },
        SessionState::STATUS => {
            Err("Not implemented.".to_string())
        },
        SessionState::LOGIN => {
            Err("Not implemented.".to_string())
        }
    }
}