use std::io::Read;
use std::net::TcpStream;
use openssl::rsa::Padding;
use crate::datatype::varint;
use crate::packet::{PacketBody, ServerBoundPacketBody};
use crate::session::Session;

#[derive(Debug)]
pub struct C2SEncryptionResponse {
    pub shared_secret_length: varint::VarInt,
    pub decrypted_shared_secret: Vec<u8>,
    pub verify_token_length: varint::VarInt,
    pub decrypted_verify_token: Vec<u8>
}

impl C2SEncryptionResponse {
    pub const PACKET_ID: i32 = 0x01;

    const NEXT_PACKET_IDS: [i32; 0] = []; // terminate connection

    fn decrypt_byte_array(session: &mut Session, from: &[u8], to: &mut [u8]) -> Result<(), std::string::String> {
        if let Err(e) = session.rsa.as_ref().unwrap().private_decrypt(from, to, Padding::PKCS1) {
            return Err(format!("Could not decrypt byte array: {}", e));
        }
        Ok(())
    }
}

impl PacketBody for C2SEncryptionResponse {
    fn update_session(&self, session: &mut Session) {
        session.next_packet_ids = &Self::NEXT_PACKET_IDS;
        session.shared_secret = Some(self.decrypted_shared_secret.clone());
    }
}

impl ServerBoundPacketBody for C2SEncryptionResponse {
    fn read_from_stream(session: &mut Session, stream: &mut impl Read) -> Result<Box<dyn ServerBoundPacketBody>, std::string::String> {
        let shared_secret_length = varint::read_from_stream(stream).unwrap();
        let mut shared_secret: Vec<u8> = vec![0; shared_secret_length.value as usize];
        if let Err(e) = stream.read_exact(&mut shared_secret) {
            return Err(format!("Failed to read shared secret: {}", e));
        };

        let verify_token_length = varint::read_from_stream(stream).unwrap();
        let mut verify_token: Vec<u8> = vec![0; verify_token_length.value as usize];
        if let Err(e) = stream.read_exact(&mut verify_token) {
            return Err(format!("Failed to read verify token: {}", e));
        };

        // decrypt shared secret
        let mut decrypted_shared_secret: Vec<u8> = vec![0; 128];
        Self::decrypt_byte_array(session, &shared_secret, &mut decrypted_shared_secret)?;
        decrypted_shared_secret.resize(16, 0);

        // decrypt verify token
        let mut decrypted_verify_token: Vec<u8> = vec![0; 128];
        Self::decrypt_byte_array(session, &verify_token, &mut decrypted_verify_token)?;
        decrypted_verify_token.resize(4, 0);

        // check verify token
        if decrypted_verify_token != *session.verify_token.as_ref().unwrap() {
            return Err(format!("Verify token is invalid."));
        }

        Ok(Box::new(C2SEncryptionResponse {
            shared_secret_length,
            decrypted_shared_secret,
            verify_token_length,
            decrypted_verify_token
        }))
    }

    fn respond(&self, session: &mut Session, stream: &mut TcpStream) -> Result<(), String> {
        Ok(())
    }
}
