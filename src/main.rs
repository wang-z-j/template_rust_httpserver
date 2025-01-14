mod models;
mod server;
use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use server::{handler, router::router_handler};

fn main() {
    let server_address = "127.0.0.1:7878";
    match start_server(server_address) {
        Ok(_) => {
            println!("Server started successfully!");
        }
        Err(_) => {
            println!("Failed to start server!");
        }
    }
}

fn start_server(address: &str) -> io::Result<()> {
    // // 创建监听器，监听给定地址
    let listener = TcpListener::bind(address)?;
    println!("Starting server at address: {}", address);
    // 接受请求并处理
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 使用线程处理每个请求
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    // 创建缓冲区
    // let mut buf = [0; 512];
    // // 读取客户端发送的数据
    // stream.read(&mut buf)?;
    // // 将字节流转为字符串
    // let request = String::from_utf8_lossy(&buf);
    // println!("Request:\n{}", request);
    // handler.
    // // 简单解析请求
    // let response =
    //     "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!";
    // // 发送响应
    // stream.write_all(response.as_bytes())?;
    // Ok(())
    router_handler(stream)
}
