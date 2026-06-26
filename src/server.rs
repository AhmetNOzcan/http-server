use crate::http::{Request, request};
use std::convert::TryFrom;
use std::{io::Read, net::TcpListener};
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection established with {}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let request = String::from_utf8_lossy(&buffer);
                            println!("Request: {}", request);
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(_) => {}
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
