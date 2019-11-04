use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

type Addr = (String, u32);

enum HTTP {
    Get,
    Post,
    Put,
    Delete
}

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

fn parse_request(request_method: &str) -> HTTP {
    if request_method == "GET" {
        return HTTP::Get;
    }
    if request_method == "POST" {
        return HTTP::Post;
    }
    if request_method == "PUT" {
        return HTTP::Put;
    }
    return HTTP::Delete;
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let req_method = request
        .split_whitespace()
        .next()
        .unwrap_or("");
    match parse_request(&req_method) {
        HTTP::Get => println!("GET"),
        HTTP::Post => println!("POST"),
        HTTP::Put => println!("PUT"),
        HTTP::Delete => println!("DELETE")
    }
}
