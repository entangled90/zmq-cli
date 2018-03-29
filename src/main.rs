extern crate regex;
extern crate zmq;
use std::env;
use regex::Regex;
use std::vec::Vec;
use std::io::{self, Read};
use zmq::{Socket, Message};

fn print_help() {
    println!("Usage: zmq-cli (PUB|SUB) tcp://url:port");
}
static MISSING_ARGUMENTS: &'static str = "missing arguments";

// fn connect_to_socket(String pattern, )
fn parse_arguments<'a>(args: &'a Vec<String>) -> Result<(zmq::SocketType, &String), &str> {
    match args.len() {
        // no args
        1 => {
            print_help();
            Err(MISSING_ARGUMENTS)
        }
        2 => {
            println!("Missing url & port!");
            print_help();
            Err(MISSING_ARGUMENTS)
        }
        3 => {
            let pattern: &str = &args[1];
            let address: &String = &args[2];

            let parsed_pattern = match pattern {
                "PUB" => Ok(zmq::PUB),
                "SUB" => Ok(zmq::SUB),
                _ => {
                    println!("pattern not supported. Use PUB or SUB.");
                    Err("Pattern not supported")
                }
            };

            let address_regex = Regex::new(r"^tcp://(.*):(.*)$").unwrap();
            let parsed_address = if address_regex.is_match(address) {
                Ok(address)
            } else {
                Err("Failed to parse address")
            };

            println!(
                "Connecting with pattern {} to address {:?}",
                pattern, parsed_address
            );

            parsed_pattern.and_then(|pattern| parsed_address.and_then(|addr| Ok((pattern, addr))))
        }
        _ => {
            println!("too many arguments {:?}", args);
            print_help();
            Err(MISSING_ARGUMENTS)
        }
    }
}

fn connect_to_socket(address: &str, pattern: zmq::SocketType) -> Result<zmq::Socket, &str> {
    println!("Connecting to socket");

    let ctx = zmq::Context::new();

    let socket = ctx.socket(pattern).unwrap();
    Ok(socket)
}

fn handle_pub(socket: Socket, addr: &str) {
    println!("{:?}", socket.get_events());
    socket.bind(addr).expect("failed to bind");
    let mut buffer = String::new();
    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let mut split = buffer.split(" ");
                if let Some(key) = split.next(){   
                    if let Some(value) = split.next(){
                        let mut sanitized = String::pop(&mut String::from(value.clone()));
                        socket.send_str(key, zmq::SNDMORE).expect("failed to send key");
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

fn handle_sub(socket: Socket, address: &str) {
    socket.connect(address).expect("Failed to connect to address");
    socket.set_subscribe("".as_bytes()).expect("Failed to subscribe!");
    loop{
        let message = socket.recv_multipart(0).expect("Failed to receive multipart message");
        let mut parts = message.into_iter();
        if let Some(key) = parts.next(){
            if let Some(value) = parts.next(){
                println!("{:?} - {:?}", String::from_utf8(key).unwrap(), String::from_utf8(value).unwrap());
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_arguments(&args) {
        Ok((pattern, addr)) => {
            let socket = connect_to_socket(addr, pattern).unwrap();
            match pattern {
                zmq::PUB => {
                    handle_pub(socket,addr);
                }
                zmq::SUB => {
                    handle_sub(socket, addr);
                }
                _ => println!(),
            }
        }
        Err(err) => println!("Impossible to continue, found {:?}", err),
    }
}
