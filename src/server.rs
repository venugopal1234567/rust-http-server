use std::net::TcpListener;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::status_code;
use crate::http::{Request, Response, StatusCode};
use std::io::{Read, Write};

pub struct Server {
    addr: String,
}

impl Server{
    pub fn new(addr: String) -> Self {
        Server { 
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listner = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listner.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                           println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                           let response =  match Request::try_from(&buffer[..]) {
                                Ok(reuest) => {
                                    dbg!(reuest);
                                    Response::new(StatusCode::Ok,
                                    Some("<h1> It works </h1>".to_string()) )
                                }
                                Err(e) => {
                                    println!("Failed to read from connection: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to read from connection: {}", e)
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from conection {}", e)
                        }
                    }
                    println!("Ok");
                },
                Err(e)=> {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}