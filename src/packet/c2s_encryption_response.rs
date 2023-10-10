use std::io::Read;
use std::net::TcpStream;
use openssl::rsa::Padding;
use crate::datatype::varint;
use crate::packet::{ClientBoundPacketBody, PacketBody, s2c_disconnect, ServerBoundPacketBody};
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

    fn read_byte_array(stream: &mut impl Read, length: usize) -> Result<Vec<u8>, std::string::String> {
        let mut array: Vec<u8> = vec![0; length];
        if let Err(e) = stream.read_exact(&mut array) {
            return Err(format!("Could not read byte array: {}", e));
        }
        Ok(array)
    }

    fn decrypt_byte_array(session: &mut Session, from: &[u8]) -> Result<Vec<u8>, std::string::String> {
        let mut to: Vec<u8> = vec![0; 128];
        if let Err(e) = session.rsa.as_ref().unwrap().private_decrypt(from, &mut to, Padding::PKCS1) {
            return Err(format!("Could not decrypt byte array: {}", e));
        }
        Ok(to)
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
        let shared_secret: Vec<u8> = Self::read_byte_array(stream, shared_secret_length.value as usize)?;

        let verify_token_length = varint::read_from_stream(stream).unwrap();
        let verify_token: Vec<u8> = Self::read_byte_array(stream, verify_token_length.value as usize)?;

        // decrypt shared secret
        let mut decrypted_shared_secret: Vec<u8> = Self::decrypt_byte_array(session, &shared_secret)?;
        decrypted_shared_secret.resize(16, 0);

        // decrypt verify token
        let mut decrypted_verify_token: Vec<u8> = Self::decrypt_byte_array(session, &verify_token)?;
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
        let response_packet = s2c_disconnect::S2CDisconnectPacket::new();
        response_packet.write_to_stream(session, stream)?;
        response_packet.update_session(session);
        Ok(())
    }
}
