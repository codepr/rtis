use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::time::Instant;
use crate::indexer;

type Addr = (String, u32);

const CRLF: &str = "\r\n\r\n";
const OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const E_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const E_METHOD_NOT_ALLOWED: &str = "HTTP/1.1 405 METHOD NOT ALLOWED\r\n\r\n";

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

struct Response {
    header: String,
    body: Option<String>,
    response_time: Option<f64>
}

impl Response {

    pub fn to_json(&self) -> String {
        return match &self.body {
            Some(b) => {
                let mut json = String::new();
                json.push_str(&self.header);
                json.push_str("{\"response_time\": ");
                json.push_str(&format!("{},", self.response_time.unwrap_or(0.0)));
                json.push_str(&format!("\"results\": [{}", b));
                json.push_str("]}");
                json.push_str(CRLF);
                json
            },
            None => String::from(&self.header)
        };
    }
}

pub struct Server {
    addr: Addr,
    indexer: indexer::Indexer
}

impl Server {

    pub fn new(addr: Addr, indexer: indexer::Indexer) -> Server {
        Server {
            addr: addr,
            indexer: indexer
        }
    }

    pub fn serve(&mut self) {
        let addr = format!("{}:{}", self.addr.0, self.addr.1);
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 8192];
        stream.read(&mut buffer).unwrap();
        let reqstring = String::from_utf8_lossy(&buffer[..]);
        let response = match parse_request(&reqstring) {
            Ok(r) => self.handle_request(r),
            Err(e) => self.handle_error(e)
        };
        send_response(stream, response);
    }

    fn handle_request(&mut self, request: Request) -> Response {
        return match request.method {
            HTTP::Get => {
                let header = OK.to_string();
                let now = Instant::now();
                let mut results = self.indexer.search(request.body).unwrap_or(vec![]);
                results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let mut body: String = results
                    .iter()
                    .rev()
                    .fold("".to_string(), |mut s, (_, b)| {s.push_str(&format!("\"{}\",", *b)); s});
                body.pop();
                Response {
                    header: header,
                    body: Some(body),
                    response_time: Some(now.elapsed().as_secs_f64())
                }
            },
            HTTP::Post => {
                self.indexer.add(request.body);
                Response { header: OK.to_string(), body: None, response_time: None }
            },
            HTTP::Put | HTTP::Delete =>
                Response {
                    header: OK.to_string(),
                    body: None,
                    response_time: None
                },
        };
    }

    fn handle_error(&self, err: HTTPError) -> Response {
        match err {
            HTTPError::NotFound =>
                Response {
                    header: E_NOT_FOUND.to_string(),
                    body: None,
                    response_time: None
                },
            HTTPError::MethodNotAllowed =>
                Response {
                    header: E_METHOD_NOT_ALLOWED.to_string(),
                    body: None,
                    response_time: None
                }
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
    let method: Option<&HTTP> = valid_methods.get(
        &reqfields[0]
        .split_whitespace()
        .next()
        .unwrap());
    let route = match reqfields[0].split(" ").nth(1) {
        Some(r) => if r == "/" { None } else { Some(r) },
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
    let body = reqfields[1].trim_matches(char::from(0)).to_string();
    // Method not in (GET, POST, PUT, DELETE)? return 405 METHOD NOT ALLOWED
    let retval = method.map_or(
        Err(HTTPError::MethodNotAllowed),
        |m| Ok(Request { method: *m, headers: headers, body: body })
    );
    // Route present ("/" is considered empty route)? return 404 NOT FOUND
    return route.map_or(retval, |_| Err(HTTPError::NotFound));
}

fn send_response(mut stream: TcpStream, response: Response) {
    stream.write(response.to_json().as_bytes()).unwrap();
    stream.flush().unwrap();
}
