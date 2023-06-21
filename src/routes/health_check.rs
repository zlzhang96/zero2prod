use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn greet() -> impl Responder {
    format!("Hello zzl!")
}
