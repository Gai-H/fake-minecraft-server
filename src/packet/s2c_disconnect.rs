use std::io::Write;
use crate::datatype::{string, varint};
use crate::packet::{ClientBoundPacketBody, PacketBody};
use crate::session::Session;

#[derive(Debug)]
pub struct S2CDisconnectPacket {
}

impl S2CDisconnectPacket {
    pub const PACKET_ID: i32 = 0x00;

    const REASON_JSON: &'static str = "{\"text\": \"You are banned from this server.\nReason: Banned by an operator.\"}";

    pub fn new() -> S2CDisconnectPacket {
        S2CDisconnectPacket {
        }
    }
}

impl PacketBody for S2CDisconnectPacket {
    fn update_session(&self, _: &mut Session) {
    }
}

impl ClientBoundPacketBody for S2CDisconnectPacket {
    fn write_to_stream(&self, session: &mut Session, stream: &mut impl Write) -> Result<(), std::string::String> {
        let packet_id_bytes: Vec<u8> = varint::VarInt::from(S2CDisconnectPacket::PACKET_ID).into();
        let reason_bytes: Vec<u8> = string::String::from(S2CDisconnectPacket::REASON_JSON).into();

        let packet_length: i32 = packet_id_bytes.len() as i32 + reason_bytes.len() as i32;
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length).into();

        let bytes: Vec<u8> = [&packet_length_bytes[..], &packet_id_bytes[..], &reason_bytes[..]].concat();

        // encrypt
        let mut ctx = match openssl::cipher_ctx::CipherCtx::new() {
            Ok(ctx) => {
                ctx
            },
            Err(e) => {
                return Err(format!("Could not create CipherCtx instance: {}", e));
            }
        };
        if let Err(e) = ctx.encrypt_init(Some(openssl::cipher::Cipher::aes_128_cfb8()), Some(session.shared_secret.as_ref().unwrap()), Some(session.shared_secret.as_ref().unwrap())) {
            return Err(format!("Could not initialize CipherCtx instance: {}", e));
        }
        let mut encrypted_bytes: Vec<u8> = vec![];
        if ctx.cipher_update_vec(&bytes, &mut encrypted_bytes).is_err() || ctx.cipher_final_vec(&mut encrypted_bytes).is_err() {
            return Err("Could not write encrypted bytes. ".to_string());
        }

        if stream.write_all(&encrypted_bytes).is_err() {
            return Err("Could not write packet to stream.".to_string());
        }

        if stream.flush().is_err() {
            return Err("Could not flush stream.".to_string());
        }

        Ok(())
    }
}