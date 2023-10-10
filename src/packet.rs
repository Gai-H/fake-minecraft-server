use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::datatype::varint;
use crate::session::{Session, SessionState};

pub mod c2s_handshake;
pub mod c2s_status_request;
pub mod s2c_status_response;
pub mod c2s_ping_request;
pub mod s2c_ping_response;
pub mod c2s_login_start;
pub mod s2c_encryption_request;

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
}

pub trait ServerBoundPacketBody: PacketBody {
    fn read_from_stream(session: &mut Session, stream: &mut impl Read) -> Result<Box<dyn ServerBoundPacketBody>, std::string::String> where Self: Sized;

    fn respond(&self, session: &mut Session, stream: &mut TcpStream) -> Result<(), String>;
}

pub trait ClientBoundPacketBody: PacketBody {
    fn write_to_stream(&self, stream: &mut impl Write) -> Result<(), std::string::String>;
}

pub fn read_packet_body_from_stream(stream: &mut TcpStream, session: &mut Session, header: &PacketHeader) -> Result<Box<dyn ServerBoundPacketBody>, std::string::String> {
    return match session.state {
        SessionState::HANDSHAKING => {
            match header.id {
                // 0x00
                c2s_handshake::C2SHandshakePacket::PACKET_ID => {
                    let packet = c2s_handshake::C2SHandshakePacket::read_from_stream(session, stream).unwrap();
                    Ok(packet)
                },
                _ => {
                    Err(format!("Invalid packet id {}", header.id))
                }
            }
        },
        SessionState::STATUS => {
            match header.id {
                // 0x00
                c2s_status_request::C2SStatusRequestPacket::PACKET_ID => {
                    let packet = c2s_status_request::C2SStatusRequestPacket::read_from_stream(session, stream).unwrap();
                    Ok(packet)
                },
                // 0x01
                c2s_ping_request::C2SPingRequestPacket::PACKET_ID => {
                    let packet = c2s_ping_request::C2SPingRequestPacket::read_from_stream(session, stream).unwrap();
                    Ok(packet)
                },
                _ => {
                    Err(format!("Invalid packet id {}", header.id))
                }
            }
        },
        SessionState::LOGIN => {
            match header.id {
                // 0x00
                c2s_login_start::C2SLoginStartPacket::PACKET_ID => {
                    let packet = c2s_login_start::C2SLoginStartPacket::read_from_stream(session, stream).unwrap();
                    Ok(packet)
                },
                _ => {
                    Err(format!("Invalid packet id {}", header.id))
                }
            }
        }
    }
}