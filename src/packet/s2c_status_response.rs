use crate::datatype::{string, varint};
use crate::packet::{ClientBoundPacketBody, PacketBody, PacketError};
use crate::session::Session;
use crate::{packet, CONFIG};
use std::io::Write;

#[derive(Debug)]
pub struct S2CStatusResponsePacket {}

impl S2CStatusResponsePacket {
    pub const PACKET_ID: i32 = 0x00;

    const DEFAULT_VERSION_NAME: &'static str = "1.20.2";
    const DEFAULT_VERSION_PROTOCOL: u16 = 764;
    const DEFAULT_DESCRIPTION: &'static str = "A Minecraft Server";
    const DEFAULT_PLAYERS_MAX: u16 = 20;
    const DEFAULT_PLAYERS_ONLINE: u16 = 0;

    pub fn new() -> S2CStatusResponsePacket {
        S2CStatusResponsePacket {}
    }

    fn get_response_json() -> String {
        let version_name = CONFIG
            .get::<String>("version_name")
            .unwrap_or(Self::DEFAULT_VERSION_NAME.into());
        let version_protocol = CONFIG
            .get::<u16>("version_protocol")
            .unwrap_or(Self::DEFAULT_VERSION_PROTOCOL);
        let description = CONFIG
            .get::<String>("description")
            .unwrap_or(Self::DEFAULT_DESCRIPTION.into());
        let players_max = CONFIG
            .get::<u16>("players_max")
            .unwrap_or(Self::DEFAULT_PLAYERS_MAX);
        let players_online = CONFIG
            .get::<u16>("players_online")
            .unwrap_or(Self::DEFAULT_PLAYERS_ONLINE);

        format!("{{\"version\":{{\"name\":\"{}\",\"protocol\":{}}},\"enforcesSecureChat\":true,\"description\":{{\"text\":\"{}\"}},\"players\":{{\"max\":{},\"online\":{}}}}}",
            version_name,
            version_protocol,
            description,
            players_max,
            players_online
        )
    }
}

impl PacketBody for S2CStatusResponsePacket {
    fn update_session(&self, _: &mut Session) {}
}

impl ClientBoundPacketBody for S2CStatusResponsePacket {
    fn write_to_stream(&self, _: &mut Session, stream: &mut impl Write) -> packet::Result<()> {
        let packet_id_bytes: Vec<u8> =
            varint::VarInt::from(S2CStatusResponsePacket::PACKET_ID).into();

        let response_json_bytes: Vec<u8> = string::String::from(Self::get_response_json()).into();

        let packet_length: usize = packet_id_bytes.len() + response_json_bytes.len();
        let packet_length_bytes: Vec<u8> = varint::VarInt::from(packet_length as i32).into();

        let bytes: Vec<u8> = [
            &packet_length_bytes[..],
            &packet_id_bytes[..],
            &response_json_bytes[..],
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
