use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").route("/hi", web::get().to(hi_dad))))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

async fn hi_dad() -> impl Responder {
    HttpResponse::Ok().body("Hi dad!")
}
