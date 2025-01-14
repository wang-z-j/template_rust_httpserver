use std::{
    collections::HashMap,
    io,
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

use crate::server::router::router_handler;

pub fn start_server(address: &str) -> io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("Starting server at address: {}", address);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || router_handler(stream));
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
