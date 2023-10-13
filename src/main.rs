mod datatype;
mod packet;
mod session;

use std::error;
use std::net::{TcpListener, TcpStream};
use config::Config;
use lazy_static::lazy_static;
use crate::session::Session;

lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("Config").required(false))
        .build()
        .unwrap();
}

fn main() {
    let port = CONFIG.get::<u16>("port").unwrap_or(25565);
    let full_address = format!("127.0.0.1:{}", port);

    let listener = match TcpListener::bind(&full_address) {
        Ok(l) => l,
        Err(_) => {
            eprintln!("Could not start listening on {}.", &full_address);
            return;
        }
    };
    println!("Successfully listening on {}.", &full_address);

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
