use fake_minecraft_server::encryption;
use std::fmt;
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub enum SessionState {
    HANDSHAKING,
    STATUS,
    LOGIN,
    // CONFIGURATION,
    // PLAY
}

impl fmt::Display for SessionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionState::HANDSHAKING => write!(f, "HANDSHAKING"),
            SessionState::STATUS => write!(f, "STATUS"),
            SessionState::LOGIN => write!(f, "LOGIN"),
        }
    }
}

#[derive(Debug)]
pub struct Session {
    pub peer_address: SocketAddr,
    pub state: SessionState,
    pub next_packet_ids: &'static [i32],
    pub protocol_version: Option<i32>,
    pub server_address: Option<String>,
    pub server_port: Option<u16>,
    pub username: Option<String>,
    pub uuid: Option<u128>,
    pub rsa: Option<encryption::Rsa>,
    pub verify_token: Option<Vec<u8>>,
    pub shared_secret: Option<Vec<u8>>,
}

impl Session {
    pub const FIRST_PACKET_IDS: [i32; 1] = [0x00]; // Handshake

    pub fn new(stream: &TcpStream) -> Session {
        Session {
            peer_address: stream.peer_addr().unwrap(),
            state: SessionState::HANDSHAKING,
            next_packet_ids: &Session::FIRST_PACKET_IDS,
            protocol_version: None,
            server_address: None,
            server_port: None,
            username: None,
            uuid: None,
            rsa: None,
            verify_token: None,
            shared_secret: None,
        }
    }
}
