use super::datatype::varint;
use super::{
    s2c_disconnect, ClientBoundPacketBody, PacketBody, PacketError, Result, ServerBoundPacketBody,
};
use crate::session::Session;
use fake_minecraft_server::encryption;
use std::io::Read;
use std::net::TcpStream;

#[derive(Debug)]
pub struct C2SEncryptionResponse {
    pub shared_secret_length: varint::VarInt,
    pub decrypted_shared_secret: Vec<u8>,
    pub verify_token_length: varint::VarInt,
    pub decrypted_verify_token: Vec<u8>,
    pub is_authenticated: bool,
}

impl C2SEncryptionResponse {
    pub const PACKET_ID: i32 = 0x01;

    const NEXT_PACKET_IDS: [i32; 0] = []; // terminate connection

    fn read_byte_array(stream: &mut impl Read, length: usize) -> Result<Vec<u8>> {
        let mut array: Vec<u8> = vec![0; length];
        if let Err(e) = stream.read_exact(&mut array) {
            return Err(PacketError::ReadError(format!("Could not read byte array: {}", e)).into());
        }
        Ok(array)
    }
}

impl PacketBody for C2SEncryptionResponse {
    fn update_session(&self, session: &mut Session) {
        session.next_packet_ids = &Self::NEXT_PACKET_IDS;
        session.shared_secret = Some(self.decrypted_shared_secret.clone());
        session.is_authenticated = self.is_authenticated;
    }
}

impl ServerBoundPacketBody for C2SEncryptionResponse {
    fn read_from_stream(
        session: &mut Session,
        stream: &mut impl Read,
    ) -> Result<Box<dyn ServerBoundPacketBody>> {
        let shared_secret_length = varint::read_from_stream(stream)?;
        let shared_secret: Vec<u8> =
            Self::read_byte_array(stream, shared_secret_length.value as usize)?;

        let verify_token_length = varint::read_from_stream(stream)?;
        let verify_token: Vec<u8> =
            Self::read_byte_array(stream, verify_token_length.value as usize)?;

        // decrypt shared secret
        let mut decrypted_shared_secret = session
            .rsa
            .as_ref()
            .unwrap()
            .decrypt_bytes(&shared_secret)?;
        decrypted_shared_secret.resize(16, 0);

        // decrypt verify token
        let mut decrypted_verify_token =
            session.rsa.as_ref().unwrap().decrypt_bytes(&verify_token)?;
        decrypted_verify_token.resize(4, 0);

        // check verify token
        if decrypted_verify_token != *session.verify_token.as_ref().unwrap() {
            return Err(PacketError::EncryptionError("Invalid verify token".to_string()).into());
        }

        // authenticate
        let auth_res = encryption::authenticate(
            &decrypted_shared_secret,
            &session
                .rsa
                .as_ref()
                .unwrap()
                .get_public_key_in_der()
                .unwrap(),
            &session.username.as_ref().unwrap(),
        );

        Ok(Box::new(C2SEncryptionResponse {
            shared_secret_length,
            decrypted_shared_secret,
            verify_token_length,
            decrypted_verify_token,
            is_authenticated: auth_res.is_ok(),
        }))
    }

    fn respond(&self, session: &mut Session, stream: &mut TcpStream) -> Result<()> {
        let response_packet = s2c_disconnect::S2CDisconnectPacket::new();
        response_packet.write_to_stream(session, stream)?;
        response_packet.update_session(session);
        Ok(())
    }
}
