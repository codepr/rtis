use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

type Addr = (String, u32);

const CRLF: &str = "\r\n\r\n";

#[derive(Copy, Clone, Debug)]
enum HTTP {
    Get,
    Post,
    Put,
    Delete
}

#[derive(Debug)]
struct Request {
    method: HTTP,
    route: String,
    headers: HashMap<String, String>,
    body: String
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

fn parse_request(request: &str) -> Request {
    let valid_methods: HashMap<&str, HTTP> =
        [("GET", HTTP::Get),
         ("POST", HTTP::Post),
         ("PUT", HTTP::Put),
         ("DELETE", HTTP::Delete)].iter().cloned().collect();
    let reqfields: Vec<&str> = request.split(CRLF).collect();
    let method = valid_methods.get(
        &reqfields[0]
        .split_whitespace()
        .next()
        .unwrap_or(""));
    let mut headers: HashMap<String, String> = HashMap::new();
    let hdr_lines: Vec<&str> = reqfields[0].split("\r\n").collect();
    for i in 1..hdr_lines.len() {
        let kv: Vec<&str> = hdr_lines[i].split(":").collect();
        headers.insert(kv[0].to_string(), kv[1].to_string());
    }
    println!("{}", reqfields[1].to_string());
    let req: Request = Request {
        method: *method.unwrap(),
        route: reqfields[0].split(" ").nth(1).unwrap().to_string(),
        headers: headers,
        body: reqfields[1].trim_matches(char::from(0)).to_string()
    };

    return req;
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let reqstring = String::from_utf8_lossy(&buffer[..]);
    let request = parse_request(&reqstring);
    println!("{:?}", request);
}
