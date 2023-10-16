use crate::session::{Session, SessionState};
use datatype::varint;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{error, fmt};

pub mod c2s_encryption_response;
pub mod c2s_handshake;
pub mod c2s_login_start;
pub mod c2s_ping_request;
pub mod c2s_status_request;
pub mod s2c_disconnect;
pub mod s2c_encryption_request;
pub mod s2c_ping_response;
pub mod s2c_status_response;

mod datatype;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct PacketHeader {
    pub length: i32,
    pub id: i32,
}

impl fmt::Display for PacketHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}, length: {}", self.id, self.length)
    }
}

pub fn read_packet_header_from_stream(
    session: &mut Session,
    stream: &mut TcpStream,
) -> Result<PacketHeader> {
    let packet_length = varint::read_from_stream(stream)?;
    let packet_id = varint::read_from_stream(stream)?;

    if !session.next_packet_ids.contains(&packet_id.value) {
        return Err(PacketError::SequenceError(format!(
            "Invalid packet order: {}",
            packet_id.value
        ))
        .into());
    }

    Ok(PacketHeader {
        length: packet_length.value,
        id: packet_id.value,
    })
}

pub trait PacketBody: Debug {
    fn update_session(&self, session: &mut Session);
}

pub trait ServerBoundPacketBody: PacketBody {
    fn read_from_stream(
        session: &mut Session,
        stream: &mut impl Read,
    ) -> Result<Box<dyn ServerBoundPacketBody>>
    where
        Self: Sized;

    fn respond(&self, session: &mut Session, stream: &mut TcpStream) -> Result<()>;
}

pub trait ClientBoundPacketBody: PacketBody {
    fn write_to_stream(&self, session: &mut Session, stream: &mut impl Write) -> Result<()>;
}

pub fn read_packet_body_from_stream(
    session: &mut Session,
    stream: &mut TcpStream,
    header: &PacketHeader,
) -> Result<Box<dyn ServerBoundPacketBody>> {
    return match session.state {
        SessionState::HANDSHAKING => {
            match header.id {
                // 0x00
                c2s_handshake::C2SHandshakePacket::PACKET_ID => {
                    let packet =
                        c2s_handshake::C2SHandshakePacket::read_from_stream(session, stream)?;
                    Ok(packet)
                }
                _ => Err(
                    PacketError::SequenceError(format!("Invalid packet id: {}", header.id)).into(),
                ),
            }
        }
        SessionState::STATUS => {
            match header.id {
                // 0x00
                c2s_status_request::C2SStatusRequestPacket::PACKET_ID => {
                    let packet = c2s_status_request::C2SStatusRequestPacket::read_from_stream(
                        session, stream,
                    )?;
                    Ok(packet)
                }
                // 0x01
                c2s_ping_request::C2SPingRequestPacket::PACKET_ID => {
                    let packet =
                        c2s_ping_request::C2SPingRequestPacket::read_from_stream(session, stream)?;
                    Ok(packet)
                }
                _ => Err(
                    PacketError::SequenceError(format!("Invalid packet id: {}", header.id)).into(),
                ),
            }
        }
        SessionState::LOGIN => {
            match header.id {
                // 0x00
                c2s_login_start::C2SLoginStartPacket::PACKET_ID => {
                    let packet =
                        c2s_login_start::C2SLoginStartPacket::read_from_stream(session, stream)?;
                    Ok(packet)
                }
                //0x01
                c2s_encryption_response::C2SEncryptionResponse::PACKET_ID => {
                    let packet = c2s_encryption_response::C2SEncryptionResponse::read_from_stream(
                        session, stream,
                    )?;
                    Ok(packet)
                }
                _ => Err(
                    PacketError::SequenceError(format!("Invalid packet id: {}", header.id)).into(),
                ),
            }
        }
    };
}

#[derive(Debug)]
pub enum PacketError {
    WriteError,
    FlushError,
    ReadError(String),
    SequenceError(String),
    EncryptionError(String),
}

impl fmt::Display for PacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PacketError::WriteError => write!(f, "Could not write packet to stream"),
            PacketError::FlushError => write!(f, "Could not flush stream"),
            PacketError::ReadError(s) => write!(f, "Read Error: {}", s),
            PacketError::SequenceError(s) => write!(f, "Sequence Error: {}", s),
            PacketError::EncryptionError(s) => write!(f, "Encryption Error: {}", s),
        }
    }
}

impl error::Error for PacketError {}
