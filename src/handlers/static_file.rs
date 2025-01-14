use actix_web::{HttpResponse, Responder};

pub async fn static_file() -> impl Responder {
    HttpResponse::Ok().body("This is a static file response")
}
