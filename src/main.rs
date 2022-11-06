#![allow(dead_code)]
use http::Method;
use http::Request;
use website_handler::WebSiteHandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    println!("{}",default_path);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = server::Server::new("127.0.0.1:8000".to_string());
    server.run(WebSiteHandler::new(public_path));
}
