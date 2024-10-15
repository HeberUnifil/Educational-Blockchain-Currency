use actix_web::*;

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body("Conectado...") 
}