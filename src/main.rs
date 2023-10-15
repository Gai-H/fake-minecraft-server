mod datatype;
mod packet;
mod session;

use crate::session::Session;
use config::Config;
use lazy_static::lazy_static;
use std::error;
use std::net::{TcpListener, TcpStream};
use std::process::Command;

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
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        let mut session = Session::new(&stream);

        if let Err(e) = handle_connection(&mut session, &mut stream) {
            eprintln!("{}", e);
            continue;
        }

        run_command(&session);
    }
}

fn handle_connection(
    session: &mut Session,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn error::Error>> {
    loop {
        // read header
        let header = packet::read_packet_header_from_stream(session, stream)?;

        // read body
        let body = packet::read_packet_body_from_stream(session, stream, &header)?;

        // update session and respond
        body.update_session(session);
        body.respond(session, stream)?;

        // terminate
        if session.next_packet_ids.len() == 0 {
            break;
        }
    }
    Ok(())
}

fn run_command(session: &Session) {
    let cmd_vec = match CONFIG.get::<Vec<String>>("command") {
        Ok(c) => c,
        Err(_) => {
            return;
        }
    };

    // replace variables
    let mut replaced_args: Vec<String> = vec![];
    for arg in &cmd_vec[1..] {
        let replaced_arg: String = match &arg[..] {
            "%peer_address%" => session.peer_address.to_string(),
            "%username%" => session.username.clone().unwrap_or("%username%".to_string()),
            "%uuid%" => {
                if session.uuid.is_none() {
                    "%uuid%".into()
                } else {
                    format!("{:x}", session.uuid.clone().unwrap())
                }
            }
            "%state%" => session.state.to_string(),
            "%is_authenticated%" => session.is_authenticated.to_string(),
            _ => arg.clone(),
        };

        replaced_args.push(replaced_arg);
    }

    // run
    let out = Command::new(&cmd_vec[0])
        .args(replaced_args)
        .output()
        .expect("Failed to run command.");
    println!("{}", String::from_utf8_lossy(&out.stdout));
    eprintln!("{}", String::from_utf8_lossy(&out.stderr));
}
