use std::fs;

use crate::http::{Method, Request, Response, StatusCode};

use super::server::Handler;

pub struct WebSiteHandler {
    public_path: String,
}

impl WebSiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, path: &str) -> Option<String> {
        let full_path = format!("{}/{}", self.public_path, path);
        match fs::canonicalize(full_path) {
            Ok(canonical_path) => {
                if !canonical_path.starts_with(&self.public_path) {
                    return None;
                } else {
                    std::fs::read_to_string(canonical_path).ok()
                }
            }
            Err(_) => return None,
        }
    }
}

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
