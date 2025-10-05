use actix_web::{web, App, HttpResponse, HttpServer, Responder};
//use blueberryjobs::controllers::candidate_controller;
//
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Iniciando servidor...");
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            //.configure(candidate_controller::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    await
}
