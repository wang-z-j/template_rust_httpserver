use crate::routes::apis::configure_routes;
use actix_web::{web, App, HttpServer};

mod errors;
mod handlers;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 启动服务器并配置路由
    HttpServer::new(|| {
        App::new().configure(configure_routes) // 使用路由配置
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
