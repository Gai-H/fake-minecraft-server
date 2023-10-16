use super::datatype::{string, uuid};
use super::{
    s2c_encryption_request, ClientBoundPacketBody, PacketBody, Result, ServerBoundPacketBody,
};
use crate::session::Session;
use std::io::Read;
use std::net::TcpStream;

#[derive(Debug)]
pub struct C2SLoginStartPacket {
    pub name: string::String,
    pub uuid: uuid::UUID,
}

impl C2SLoginStartPacket {
    pub const PACKET_ID: i32 = 0x00;

    const NEXT_PACKET_IDS: [i32; 1] = [0x01]; // Encryption Response
}

impl PacketBody for C2SLoginStartPacket {
    fn update_session(&self, session: &mut Session) {
        session.username = Some(self.name.value.clone());
        session.uuid = Some(self.uuid.value.clone());
        session.next_packet_ids = &Self::NEXT_PACKET_IDS;
    }
}

impl ServerBoundPacketBody for C2SLoginStartPacket {
    fn read_from_stream(
        _: &mut Session,
        stream: &mut impl Read,
    ) -> Result<Box<dyn ServerBoundPacketBody>> {
        let name = string::read_from_stream(stream)?;

        let uuid = uuid::read_from_stream(stream)?;

        Ok(Box::new(C2SLoginStartPacket { name, uuid }))
    }

    fn respond(&self, session: &mut Session, stream: &mut TcpStream) -> Result<()> {
        let response_packet = s2c_encryption_request::S2CEncryptionRequest::new()?;

        response_packet.update_session(session);

        response_packet.write_to_stream(session, stream).into()
    }
}
