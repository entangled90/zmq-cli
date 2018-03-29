extern crate zmq;
extern crate regex;
use std::env;
use regex::Regex;
use std::vec::Vec;

fn print_help(){
    println!("Usage: zmq-cli (PUB|SUB) tcp://url:port");
}
static MISSING_ARGUMENTS:  &'static str = "missing arguments";

// fn connect_to_socket(String pattern, )
fn parse_arguments<'a>(args: &'a Vec<String>) -> Result<(zmq::SocketType,&String), &str> {
    match args.len() {
        // no args
        1 =>{
            print_help();
            Err(MISSING_ARGUMENTS)
        },
        2 =>{ 
            println!("Missing url & port!");
            print_help();
            Err(MISSING_ARGUMENTS)
        },
        3 =>{
            let pattern: &str = &args[1];
            let address : &String = &args[2];

            let parsed_pattern = match pattern{
                "PUB" =>
                    Ok(zmq::PUB),
                "SUB" =>
                    Ok(zmq::SUB),
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

            println!("Connecting with pattern {} to address {:?}", pattern, parsed_address );

            parsed_pattern.and_then(|pattern| {
                parsed_address.and_then(|addr|{
                Ok((pattern,addr))
                })
            })
        },
        _ =>{
            println!("too many arguments {:?}", args );
            print_help();
            Err(MISSING_ARGUMENTS)
        } 
          
        }
}

fn connect_to_socket(address: &str, pattern: zmq::SocketType) -> Result<&str,&str> {
    println!("Connecting to socket");
     
    let ctx = zmq::Context::new();

    let socket = ctx.socket(pattern).unwrap();
    socket.connect(address).unwrap();
    Ok("Connected")
}
fn main() {

    let args: Vec<String> = env::args().collect();

    match parse_arguments(&args) {
        Ok((pattern, addr)) =>{
            connect_to_socket(addr, pattern);
        },
        Err(err) =>
        println!("Impossible to continue, found {:?}", err)
    }
    

    // let ctx = zmq::Context::new();

    // let mut socket = ctx.socket(zmq::REQ).unwrap();
    // socket.connect("tcp://127.0.0.1:1234").unwrap();
    // socket.send_str("hello world!", 0).unwrap();
}