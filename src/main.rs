
#[macro_use]
extern crate actix_rt;

#[macro_use]
extern crate log;


use actix_web::{HttpServer, App, HttpResponse, web, middleware::Logger};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Starting Up...");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(|| HttpResponse::Ok().body("Hello World!")))
            .route("/hello/{name}", web::get().to(hello))
    })
        .bind("localhost:8000")
        .unwrap()
        .run()
        .await
}

async fn hello(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello {}", path))
}