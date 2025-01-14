use crate::server::handler::{handle_about, handle_home, handle_not_found};
use crate::server::static_file::handle_static_file;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::TcpStream,
};

pub type RouteHandler = fn() -> (u16, String); // 定义一个类型别名，表示一个路由处理函数

pub fn router_handler(mut stream: TcpStream) -> io::Result<()> {
    let routes = init_routers();
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    let request = String::from_utf8_lossy(&buf);
    // 提取请求路径
    let path = extract_path(&request);
    println!("path:{}", path);
    let (status_code, body) = match routes.get(&path) {
        Some(handle) => handle(),
        None => handle_not_found(),
    };
    let response = format!(
        "HTTP/1.1 {} OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        body.len(),
        body
    );
    stream.write_all(response.as_bytes())?;
    Ok(())
}
fn init_routers() -> HashMap<String, RouteHandler> {
    let mut routes: HashMap<String, RouteHandler> = HashMap::new();
    routes.insert("/home".to_string(), handle_home);
    routes.insert("/about".to_string(), handle_about);
    routes.insert("/index.html".to_string(), handle_static_file);
    routes
}
fn extract_path(request: &str) -> String {
    // 假设请求的格式类似于 "GET /home HTTP/1.1"
    if let Some(start) = request.find("GET ") {
        if let Some(end) = request[start..].find(" HTTP/1.1") {
            return request[start + 4..start + end].to_string();
        }
    }
    "/".to_string() // 默认路径
}
