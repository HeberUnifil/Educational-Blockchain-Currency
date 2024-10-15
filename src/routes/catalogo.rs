use actix_web::*;

pub async fn catalogo(b: String) -> HttpResponse {
    HttpResponse::Ok()
    .content_type("application/json; charset=utf-8")
    .body(
        r#"{
        "resumo": {"qtd": 2, "livraira"}
        "livros": [
        {"msg":"teste", "autor":"lenhaslenhas"},
        {"msg":"teste", "autor":"lenhaslenhas"}]
        }"#
    )
}