extern crate clap;
extern crate regex;
extern crate zmq;
use std::env;
use regex::Regex;
use std::vec::Vec;
use std::io::{self, Read};
use zmq::{Message, Socket};
use clap::{App, Arg, SubCommand};

fn connect_to_socket(address: &str, pattern: zmq::SocketType) -> Result<zmq::Socket, &str> {
    println!("Connecting to socket");

    let ctx = zmq::Context::new();

    let socket = ctx.socket(pattern).unwrap();
    Ok(socket)
}

pub mod zmq_helpers {

    pub mod publisher {
        use std::io::{self, Read};
        use zmq::{Message, Socket, SNDMORE};
        pub fn handle_pub(socket: Socket, addr: &str) {
            println!("{:?}", socket.get_events());
            socket.bind(addr).expect("failed to bind");
            let mut buffer = String::new();
            loop {
                match io::stdin().read_line(&mut buffer) {
                    Ok(_) => {
                        let mut split = buffer.split(" ");
                        if let Some(key) = split.next() {
                            if let Some(value) = split.next() {
                                socket.send_str(key, SNDMORE).expect("failed to send key");
                                // flag 0 to indicate it's the last message
                                socket.send_str(value, 0).expect("Failed to send value");
                                println!("Sent message with key {} and value {} ", key, value);
                            }
                        }
                    }
                    Err(err) => println!("Err: {:?}", err),
                }
                buffer.clear();
            }
        }

    }

    pub mod subscriber {
        use std::io::{self, Read};
        use zmq::{Message, Socket};

        pub fn handle_sub(socket: Socket, address: &str, parse_as_string: bool) {
            socket
                .connect(address)
                .expect("Failed to connect to address");
            socket
                .set_subscribe("".as_bytes())
                .expect("Failed to subscribe!");
            loop {
                let message = socket
                    .recv_multipart(0)
                    .expect("Failed to receive multipart message");
                let mut parts = message.into_iter();
                if let Some(key) = parts.next() {
                    if parse_as_string {
                        if let Some(value) = parts.next() {
                            println!(
                                "{:?} - {:?}",
                                String::from_utf8(key).unwrap(),
                                String::from_utf8(value).unwrap()
                            );
                        }
                    } else {
                        println!("key: {:?}", String::from_utf8(key).unwrap());
                    }
                }
            }
        }
    }
}

struct Configuration<'a> {
    pattern: &'a str,
    address: &'a str,
}

fn main() {
    let matches = App::new("Zmq cli")
        .arg(
            Arg::with_name("pattern")
                .required(true)
                .possible_values(&["PUB", "SUB"])
                .help("Zmq pattern to use. Supported: [SUB|PUB]"),
        )
        .arg(
            Arg::with_name("address")
                .required(true)
                .help("Address you want to connect/bind"),
        )
        .get_matches();

    println!("arg matches {:?}", matches);

    let parsed_configuration = matches.value_of("pattern").and_then(|p| {
        matches.value_of("address").map(|a| Configuration {
            pattern: p,
            address: a,
        })
    });

    if let Some(Configuration {
        pattern: patt,
        address: addr,
    }) = parsed_configuration{
        let pattern = match patt {
            "SUB" => zmq::SUB,
            "PUB" => zmq::PUB,
            _ => panic!("impossible patern found"),
        };

        let socket = connect_to_socket(addr, pattern).unwrap();
        match pattern {
            zmq::PUB => {
                zmq_helpers::publisher::handle_pub(socket, addr);
            }
            zmq::SUB => {
                zmq_helpers::subscriber::handle_sub(socket, addr, false);
            }
            _ => println!(),
        }
    }
}
