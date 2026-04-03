mod http_request;
mod http_response;
mod routes;
mod traits;

use crate::{http_request::HttpRequest, http_response::HttpResponse, traits::New};
use std::{collections::HashMap, fs::File, io::Read, net::TcpListener};

fn root_handler(_request: &HttpRequest) -> std::io::Result<HttpResponse> {
    let mut file = File::open("./index.html")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut response = HttpResponse {
        status_code: "200".to_string(),
        http_version: "1.1".to_string(),
        headers: HashMap::new(),
        body: content.to_string(),
    };
    response
        .headers
        .insert("Content-Type".to_string(), "text/html".to_string());
    response.headers.insert(
        "Content-Length".to_string(),
        content.as_bytes().len().to_string(),
    );

    Ok(response)
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    let mut router = routes::Routes::new();
    router.add("/".to_string(), root_handler);

    loop {
        match listener.accept() {
            Ok((mut socket, _addr)) => match http_request::handle_request(&mut socket, &router) {
                Ok(()) => {}
                Err(e) => {
                    println!("Error occured: {}", e);
                    http_response::redirect(&mut socket, "/".to_string()).unwrap()
                }
            },
            Err(e) => println!("Error occured {e:?}"),
        }
    }
}
