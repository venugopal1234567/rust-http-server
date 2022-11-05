#![allow(dead_code)]
use http::Method;
use http::Request;

mod http;
mod server;

fn main() {
    let server = server::Server::new("127.0.0.1:8000".to_string());
    server.run();
}
