#[derive(Debug)]
pub enum SessionState {
    HANDSHAKING,
    STATUS,
    LOGIN,
    // CONFIGURATION,
    // PLAY
}

#[derive(Debug)]
pub struct Session {
    pub state: SessionState,
    pub next_packet_ids: &'static [i32],
    pub protocol_version: Option<i32>,
    pub server_address: Option<String>,
    pub server_port: Option<u16>
}

impl Session {
    pub const FIRST_PACKET_IDS: [i32; 1] = [0x00]; // Handshake

    pub fn new() -> Session {
        Session {
            state: SessionState::HANDSHAKING,
            protocol_version: None,
            server_address: None,
            server_port: None,
            next_packet_ids: &Session::FIRST_PACKET_IDS
        }
    }
}