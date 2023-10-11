use std::io::Write;
use crate::datatype::{string, varint};
use crate::packet;
use crate::packet::{ClientBoundPacketBody, PacketBody, PacketError};
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
    fn write_to_stream(&self, session: &mut Session, stream: &mut impl Write) -> packet::Result<()> {
        let packet_id_bytes: Vec<u8> = varint::VarInt::from(S2CDisconnectPacket::PACKET_ID).into();
        let reason_bytes: Vec<u8> = string::String::from(S2CDisconnectPacket::REASON_JSON).into();

        let packet_length: usize = packet_id_bytes.len() + reason_bytes.len();
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length as i32).into();

        let bytes: Vec<u8> = [&packet_length_bytes[..], &packet_id_bytes[..], &reason_bytes[..]].concat();

        // encrypt
        let mut ctx = match openssl::cipher_ctx::CipherCtx::new() {
            Ok(ctx) => {
                ctx
            },
            Err(e) => {
                return Err(PacketError::EncryptionError(format!("Could not create CipherCtx instance: {}", e)).into());
            }
        };
        if let Err(e) = ctx.encrypt_init(Some(openssl::cipher::Cipher::aes_128_cfb8()), Some(session.shared_secret.as_ref().unwrap()), Some(session.shared_secret.as_ref().unwrap())) {
            return Err(PacketError::EncryptionError(format!("Could not initialize CipherCtx instance: {}", e)).into());
        }
        let mut encrypted_bytes: Vec<u8> = vec![];
        if  ctx.cipher_update_vec(&bytes, &mut encrypted_bytes).is_err() ||
            ctx.cipher_final_vec(&mut encrypted_bytes).is_err() ||
            stream.write_all(&encrypted_bytes).is_err()
        {
            return Err(PacketError::WriteError.into());
        }

        if stream.flush().is_err() {
            return Err(PacketError::FlushError.into());
        }

        Ok(())
    }
}