use std::{io::Read, net::TcpListener, process::Command, thread, time::Duration};

use clap::Parser;
use log::*;
use num_derive::FromPrimitive;
use num::FromPrimitive;

//use enigo::*;
//use enigo::Direction::Click;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, help = "Network port to use", default_value_t = 54321)]
    port: u16,
    #[arg(short, long, help = "Anki wrapper", default_value_t = false)]
    anki: bool,
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
enum buttonClicked {
    Up = 4,
    Down = 5,
    LongUp = 8,
    LongDown = 9,
    Unknown = 0,
}

pub fn run_ydotool(str: &str) {
    let command = format!("ydotool key {}:1 {}:0", str, str);

    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
}

// False is it's at the show screen
// True it's at the select screen
static mut ANKI_OPEN: bool = true;

pub fn anki_wrap(str: String, args: &Args) {
    //let mut _set = enigo::Settings::default();
    //let mut enigo = Enigo::new(&_set).unwrap();

    let button: buttonClicked = buttonClicked::from_u8(str.parse::<u8>().unwrap()).unwrap();
    debug!("Received button: {:?}", button);

    if unsafe { ANKI_OPEN } {
        //enigo.text(" ").unwrap();
        run_ydotool("57");
    } else {
        match button {
            buttonClicked::Up => {
                // Good
                //enigo.text("3").unwrap();
                run_ydotool("4");
            },
            buttonClicked::Down => {
                // Hard
                //enigo.text("2").unwrap();
                run_ydotool("3");
            },
            buttonClicked::LongUp => {
                // Easy
                //enigo.text("4").unwrap();
                run_ydotool("5");
            },
            buttonClicked::LongDown => {
                // Again
                //enigo.text("1").unwrap();
                run_ydotool("2");
            },
            buttonClicked::Unknown => {
                error!("Unknown button?");
            },
        }
    }
    unsafe { ANKI_OPEN = !ANKI_OPEN };
}

pub fn wrapper(str: String, args: &Args) {
    if args.anki {
        debug!("Using anki wrapper");
        anki_wrap(str, args);
    } else {
        error!("No wrapper selected!");
    }
}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug"),
    );
    debug!("Starting inkwatchy-pc-api");

    let args = Args::parse();

    let addr = format!("0.0.0.0:{}", args.port);
    info!("Listening at: {}", addr);
    let listener = TcpListener::bind(addr).expect("Could not bind to address");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                info!("New connection: {}", stream.peer_addr().unwrap());

                let mut buffer = [0; 1024];
                // Read data from the stream
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let str = String::from_utf8_lossy(&buffer[..size]).to_string();
                        debug!("Received: {}", str);
                        wrapper(str, &args);
                    }
                    Err(e) => {
                        error!("Failed to read from client: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}
