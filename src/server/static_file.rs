use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

pub fn handle_static_file() -> (u16, String) {
    let cwd: std::path::PathBuf = env::current_dir().unwrap();
    // 构造文件的绝对路径
    let path = cwd.join("src/static/index.html");
    println!("path: {:?}", path);
    // let path = "../static/index.html"; // 例如一个静态文件路径
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (200, contents),
        Err(_) => (404, "<h1>404 Not Found</h1>".to_string()),
    }
}
