use crate::handlers::{greet::greet, static_file::static_file, user::user_info};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(greet)); // 根路径处理函数
    cfg.route("/userInfo", web::get().to(user_info));
    cfg.route("/static", web::get().to(static_file)); // 静态文件路径处理函数
}
