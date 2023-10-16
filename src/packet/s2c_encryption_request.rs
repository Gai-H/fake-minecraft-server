use super::datatype::{string, varint};
use super::{ClientBoundPacketBody, PacketBody, PacketError, Result};
use crate::session::Session;
use fake_minecraft_server::encryption;
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug)]
pub struct S2CEncryptionRequest {
    pub rsa: encryption::Rsa,
    pub server_id: string::String,
    pub public_key_length: varint::VarInt,
    pub public_key: Vec<u8>,
    pub verify_token_length: varint::VarInt,
    pub verify_token: Vec<u8>,
}

impl S2CEncryptionRequest {
    pub const PACKET_ID: i32 = 0x01;

    pub fn new() -> Result<S2CEncryptionRequest> {
        let server_id = string::String::from("");

        // generate RSA
        let rsa = encryption::Rsa::new()?;
        let public_key = rsa.get_public_key_in_der()?;
        let public_key_length = varint::VarInt::from(public_key.len() as i32);

        // generate verify token
        let verify_token_length = varint::VarInt::from(4);
        let verify_token = encryption::generate_verify_token()?.to_vec();

        Ok(S2CEncryptionRequest {
            rsa,
            server_id,
            public_key_length,
            public_key,
            verify_token_length,
            verify_token,
        })
    }
}

impl PacketBody for S2CEncryptionRequest {
    fn update_session(&self, session: &mut Session) {
        session.rsa = Some(self.rsa.clone());
        session.verify_token = Some(self.verify_token.clone());
    }
}

impl ClientBoundPacketBody for S2CEncryptionRequest {
    fn write_to_stream(&self, _: &mut Session, stream: &mut impl Write) -> Result<()> {
        let packet_id_bytes: Vec<u8> = varint::VarInt::from(S2CEncryptionRequest::PACKET_ID).into();

        let server_id_bytes: Vec<u8> = self.server_id.clone().into();
        let public_key_length_bytes: Vec<u8> = self.public_key_length.clone().into();
        let verify_token_length_bytes: Vec<u8> = self.verify_token_length.clone().into();

        let packet_length: usize = [
            packet_id_bytes.len(),
            server_id_bytes.len(),
            public_key_length_bytes.len(),
            self.public_key.len(),
            verify_token_length_bytes.len(),
            self.verify_token.len(),
        ]
        .iter()
        .sum();
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length as i32).into();

        let bytes: Vec<u8> = [
            &packet_length_bytes[..],
            &packet_id_bytes[..],
            &server_id_bytes[..],
            &public_key_length_bytes[..],
            &self.public_key[..],
            &verify_token_length_bytes[..],
            &self.verify_token[..],
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
