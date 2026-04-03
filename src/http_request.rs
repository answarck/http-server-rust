use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::{Read, Result};
use std::net::TcpStream;

use crate::http_response::send_response;
use crate::routes::Routes;

pub struct HttpRequest {
    pub method: String,
    pub location: String,
    pub http_version: String,
    pub host: String,
    pub user_agent: String,
    pub header: HashMap<String, String>,
    // body: String,
}

fn validate_request(request: &HttpRequest) -> Result<()> {
    if request.method != "GET" {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Only GET request supported",
        ));
    }
    if request.http_version != "1.1" {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "HTTP/1.1 only supported",
        ));
    }
    Ok(())
}

fn parse_header(header: &str) -> std::result::Result<(&str, &str), &'static str> {
    let mut part = header.split(": ");
    let key = part.next().ok_or("Invalid Header")?;
    let value = part.next().ok_or("Invalid Header")?;

    if part.next().is_some() {
        return Err("invalid header");
    }
    Ok((key, value))
}

fn parse_request(request: &[u8]) -> Result<HttpRequest> {
    let request_string = String::from_utf8_lossy(request);

    let mut request_parsed: HttpRequest = HttpRequest {
        method: String::new(),
        location: String::new(),
        http_version: String::new(),
        host: String::new(),
        user_agent: String::new(),
        header: HashMap::new(),
        // body: String::new(),
    };

    let mut go_to_body = false;
    for (ind, line) in request_string.lines().enumerate() {
        if go_to_body {
            break;
        }
        if ind == 0 {
            let parts: Vec<&str> = line.split(' ').collect();
            for (k, part) in parts.iter().enumerate() {
                if k == 0 {
                    request_parsed.method = part.to_string();
                } else if k == 1 {
                    request_parsed.location = part.to_string();
                } else if k == 2 {
                    request_parsed.http_version = part[5..part.len()].to_string();
                }
            }
        } else if ind == 1 {
            let parts: Vec<&str> = line.split(": ").collect();
            request_parsed.host = parts[1].to_string();
        } else if ind == 2 {
            let parts: Vec<&str> = line.split(": ").collect();
            request_parsed.user_agent = parts[1].to_string();
        } else {
            if line == "" {
                go_to_body = true;
                continue;
            }
            let (header, value) =
                parse_header(line).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            request_parsed
                .header
                .insert(String::from(header), String::from(value));
        }
    }
    Ok(request_parsed)
}

pub fn handle_request(mut stream: &TcpStream, router: &Routes) -> Result<()> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer)?;

    let parsed_request = parse_request(&mut buffer)?;
    match validate_request(&parsed_request) {
        Ok(()) => send_response(&parsed_request, &stream, &router)?,
        Err(err) => return Err(err),
    }

    Ok(())
}
