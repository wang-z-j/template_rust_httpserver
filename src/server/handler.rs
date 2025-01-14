// 逻辑处理层
use std::fs;
use std::path::Path;
// handler.rs
pub fn handle_home() -> (u16, String) {
    (200, "<h1>Home Page</h1>".to_string())
}

pub fn handle_about() -> (u16, String) {
    (200, "<h1>About Page</h1>".to_string())
}

pub fn handle_not_found() -> (u16, String) {
    (404, "<h1>404 Not Found</h1>".to_string())
}
