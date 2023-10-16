mod packet;
mod session;

use crate::session::Session;
use config::Config;
use env_logger::Builder;
use env_logger::Target::Stdout;
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
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
    Builder::from_default_env().target(Stdout).init();

    let port = CONFIG.get::<u16>("port").unwrap_or(25565);
    let full_address = format!("0.0.0.0:{}", port);

    let listener = match TcpListener::bind(&full_address) {
        Ok(l) => l,
        Err(_) => {
            error!("Could not start listening on {}.", &full_address);
            return;
        }
    };
    info!("Successfully listening on {}.", &full_address);

    for stream in listener.incoming() {
        info!("New connection");
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };
        let mut session = Session::new(&stream);
        info!("[Start] {}", session.peer_address.to_string());

        if let Err(e) = handle_connection(&mut session, &mut stream) {
            error!("{}", e);
            continue;
        }

        run_command(&session);
        info!("[End] {}", session.peer_address.to_string());
    }
}

fn handle_connection(
    session: &mut Session,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn error::Error>> {
    loop {
        let header = packet::read_packet_header_from_stream(session, stream)?;
        debug!("PacketHeader: {{{}}}", header);

        let body = packet::read_packet_body_from_stream(session, stream, &header)?;
        debug!("PacketBody received");

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
    info!("Run: {} {}", &cmd_vec[0], replaced_args.join(" "));

    // run
    let out = Command::new(&cmd_vec[0])
        .args(replaced_args)
        .output()
        .expect("Failed to run command.");
    info!("StdOut: {}", String::from_utf8_lossy(&out.stdout));
    warn!("StdErr: {}", String::from_utf8_lossy(&out.stderr));
}
