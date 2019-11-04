use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

type Addr = (String, u32);

pub struct Server {
    addr: Addr
}

impl Server {

    pub fn new(addr: Addr) -> Server { Server { addr: addr } }

    pub fn start(&self) {
        let addr = format!("{}:{}", self.addr.0, self.addr.1);
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            handle_connection(stream.unwrap());
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
