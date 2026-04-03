use crate::http_request::HttpRequest;
use crate::routes::Routes;
use crate::traits::New;
use std::collections::HashMap;
use std::io::{Result, Write};
use std::net::TcpStream;

pub struct HttpResponse {
    pub status_code: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl New for HttpResponse {
    fn new() -> Self {
        HttpResponse {
            status_code: String::new(),
            http_version: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

fn validate_response_struct(response_struct: &HttpResponse) -> Result<()> {
    Ok(())
}

fn build_header_string(response: &HttpResponse) -> Result<String> {
    let mut header_string = String::new();
    for header in response.headers.keys().into_iter() {
        let value = response
            .headers
            .get(header)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "err"))?;

        header_string.push_str(&format!("{}: {}\r\n", header, value));
    }
    Ok(header_string)
}

fn build_response(response: &HttpResponse) -> Result<String> {
    validate_response_struct(response)?;
    let header_string = build_header_string(&response)?;

    let response_string = format!(
        "HTTP/{} {}\r\n{}\r\n{}",
        response.http_version, response.status_code, header_string, response.body
    );

    Ok(response_string)
}

pub fn send_response(request: &HttpRequest, mut stream: &TcpStream, router: &Routes) -> Result<()> {
    match router.route_exists(&request.location.to_string()) {
        Ok(handler) => {
            let response = handler(&request)?;
            let response_string = build_response(&response)?;
            stream.write(&response_string.as_bytes())?;
        }
        Err(e) => return Err(e),
    }
    Ok(())
}

pub fn redirect(mut stream: &TcpStream, location: String) -> Result<()> {
    let mut response: HttpResponse = HttpResponse {
        status_code: String::from("302 Found"),
        http_version: String::from("1.1"),
        headers: HashMap::new(),
        body: "".to_string(),
    };
    response.headers.insert(String::from("Location"), location);
    response
        .headers
        .insert(String::from("Content-Length"), "0".to_string());

    let response_string: String = build_response(&response)?;
    stream.write(response_string.as_bytes())?;
    Ok(())
}
