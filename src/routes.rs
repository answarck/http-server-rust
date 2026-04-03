use std::collections::HashMap;

use crate::http_request::HttpRequest;
use crate::http_response::HttpResponse;
use crate::traits::New;

pub struct Routes {
    routes: HashMap<String, fn(&HttpRequest) -> std::io::Result<HttpResponse>>,
}

impl Routes {
    pub fn add(
        &mut self,
        location: String,
        handler: fn(&HttpRequest) -> std::io::Result<HttpResponse>,
    ) {
        if self.routes.insert(location, handler).is_some() {
            panic!("Route already exists");
        }
    }

    pub fn route_exists(
        &self,
        location: &str,
    ) -> std::io::Result<&fn(&HttpRequest) -> std::io::Result<HttpResponse>> {
        println!("rout is {location}");
        self.routes.get(location).ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "route not found",
        ))
    }
}

impl New for Routes {
    fn new() -> Self {
        Routes {
            routes: HashMap::new(),
        }
    }
}
