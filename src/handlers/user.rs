use actix_web::{HttpResponse, Responder};

use crate::models::user::User;
pub async fn user_info() -> impl Responder {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "Alice@qq.com".to_string(),
    };
    HttpResponse::Ok().json(user)
}
