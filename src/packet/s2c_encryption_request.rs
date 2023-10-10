use std::fmt::Debug;
use std::io::Write;
use std::net::TcpStream;
use crate::datatype::{string, varint};
use crate::packet::{ClientBoundPacketBody, PacketBody};
use crate::session::Session;

#[derive(Debug)]
pub struct S2CEncryptionRequest {
    pub rsa: openssl::rsa::Rsa<openssl::pkey::Private>,
    pub server_id: string::String,
    pub public_key_length: varint::VarInt,
    pub public_key: Vec<u8>,
    pub verify_token_length: varint::VarInt,
    pub verify_token: Vec<u8>
}

impl S2CEncryptionRequest {
    pub const PACKET_ID: i32 = 0x01;

    pub fn new() -> Result<S2CEncryptionRequest, String> {
        let server_id = string::String::from("");

        // generate RSA key pair
        let rsa = match openssl::rsa::Rsa::generate(1024) {
            Ok(r) => r,
            Err(e) => return Err(format!("Failed to generate RSA key pair: {}", e))
        };
        let public_key = match rsa.public_key_to_der() {
            Ok(p) => p,
            Err(e) => return Err(format!("Failed to convert RSA public key to DER: {}", e))
        };
        let public_key_length = varint::VarInt::from(public_key.len() as i32);

        // generate verify token
        let verify_token_length = varint::VarInt::from(4);
        let mut verify_token_array: [u8; 4] = [0; 4];
        match openssl::rand::rand_bytes(&mut verify_token_array) {
            Ok(_) => {},
            Err(e) => return Err(format!("Failed to generate verify token: {}", e))
        };
        let verify_token = verify_token_array.to_vec();

        Ok(S2CEncryptionRequest {
            rsa,
            server_id,
            public_key_length,
            public_key,
            verify_token_length,
            verify_token
        })
    }
}

impl PacketBody for S2CEncryptionRequest {
    fn update_session(&self, session: &mut Session) {
        session.rsa = Some(self.rsa.clone());
        session.verify_token = Some(self.verify_token.clone());
    }

    fn handle(&self, _: &mut Session, _: &mut TcpStream) -> Result<(), String> {
        Ok(())
    }
}

impl ClientBoundPacketBody for S2CEncryptionRequest {
    fn write_to_stream(&self, stream: &mut impl Write) -> Result<(), std::string::String> {
        let packet_id_bytes: Vec<u8> = varint::VarInt::from(S2CEncryptionRequest::PACKET_ID).into();

        let server_id_bytes: Vec<u8> = self.server_id.clone().into();
        let public_key_length_bytes: Vec<u8> = self.public_key_length.clone().into();
        let verify_token_length_bytes: Vec<u8> = self.verify_token_length.clone().into();

        let packet_length: usize = packet_id_bytes.len() + server_id_bytes.len() + public_key_length_bytes.len() + self.public_key.len() + verify_token_length_bytes.len() + self.verify_token.len();
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length as i32).into();

        let bytes: Vec<u8> = [
            &packet_length_bytes[..],
            &packet_id_bytes[..],
            &server_id_bytes[..],
            &public_key_length_bytes[..],
            &self.public_key[..],
            &verify_token_length_bytes[..],
            &self.verify_token[..]
        ].concat();

        if stream.write_all(&bytes).is_err() {
            return Err("Could not write packet to stream.".to_string());
        }

        if stream.flush().is_err() {
            return Err("Could not flush stream.".to_string());
        }

        Ok(())
    }
}