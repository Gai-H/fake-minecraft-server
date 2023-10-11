mod datatype;
mod packet;
mod session;

use std::error;
use std::net::{TcpListener, TcpStream};
use crate::session::Session;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        if let Err(e) = handle_connection(stream) {
            eprintln!("{}", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn error::Error>> {
    let mut session = Session::new();

    loop {
        dbg!(&session);
        // read header
        let header = packet::read_packet_header_from_stream(&mut session, &mut stream)?;
        dbg!(&header);

        // read body
        let body = packet::read_packet_body_from_stream(&mut session, &mut stream, &header)?;
        dbg!(&body);

        // update session and respond
        body.update_session(&mut session);
        body.respond(&mut session, &mut stream)?;

        // terminate
        if session.next_packet_ids.len() == 0 {
            break
        }
    }
    Ok(())
}
