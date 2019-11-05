use std::thread;
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
enum HTTPError {
    NotFound = 404,
    MethodNotAllowed = 405
}

#[derive(Debug)]
struct Request {
    method: HTTP,
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
            thread::spawn(|| handle_connection(stream.unwrap()));
        }
    }
}

fn parse_request(request: &str) -> Result<Request, HTTPError> {
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
    let route = match reqfields[0].split(" ").nth(1) {
        Some(r) => {
            if r == "/" {
                None
            } else {
                Some(r)
            }
        },
        None => None
    };
    let mut headers: HashMap<String, String> = HashMap::new();
    let hdr_lines: Vec<&str> = reqfields[0].split("\r\n").collect();
    // Populate headers map, starting from 1 as index to skip the first line which
    // contains just the HTTP method and route
    for i in 1..hdr_lines.len() {
        let kv: Vec<&str> = hdr_lines[i].split(":").collect();
        headers.insert(kv[0].to_string(), kv[1].to_string());
    }
    return match route {
        Some(_) => Err(HTTPError::NotFound),
        None => {
            match method {
                Some(m) => {
                    Ok(
                        Request {
                            method: *m,
                            headers: headers,
                            body: reqfields[1].trim_matches(char::from(0)).to_string()
                        }
                    )
                },
                None => Err(HTTPError::MethodNotAllowed)
            }
        }
    };
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let reqstring = String::from_utf8_lossy(&buffer[..]);
    let response = match parse_request(&reqstring) {
        Ok(r) => handle_request(r),
        Err(e) => handle_error(e)
    };
    send_response(stream, response.as_bytes());
}

fn handle_error(err: HTTPError) -> String {
    match err {
        HTTPError::NotFound => "HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string(),
        HTTPError::MethodNotAllowed => "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n".to_string()
    }
}

fn handle_request(request: Request) -> String {
    match request.method {
        HTTP::Get => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        HTTP::Post => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        HTTP::Put => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        HTTP::Delete => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
    }
}

fn send_response(mut stream: TcpStream, response: &[u8]) {
    stream.write(response).unwrap();
    stream.flush().unwrap();
}
