use actix_web::{web, App, HttpServer, HttpResponse, Responder, dev, Result, http};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:3000")
        .unwrap()
        .run()
        .unwrap();
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}