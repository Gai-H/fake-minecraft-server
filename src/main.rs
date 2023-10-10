mod datatype;
mod packet;
mod session;

use std::net::{TcpListener, TcpStream};
use crate::session::Session;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut session = Session::new();

    loop {
        dbg!(&session);
        // read header
        let header = match packet::read_packet_header_from_stream(&mut stream) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("{}", e);
                return
            }
        };
        if !session.next_packet_ids.contains(&header.id) {
            eprintln!("Invalid packet order {}", header.id);
            return
        }
        dbg!(&header);

        // read body
        let body = match packet::read_packet_body_from_stream(&mut stream, &session, &header) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{}", e);
                return
            }
        };
        dbg!(&body);

        // update session and respond
        body.update_session(&mut session);
        match body.respond(&mut session, &mut stream) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}", e);
                return
            }
        };

        // terminate
        if session.next_packet_ids.len() == 0 {
            break
        }
    }
}
