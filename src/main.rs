
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

mod environment;
mod guards;
mod models;
mod routes;

use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = AppState::init();
    let app_data = web::Data::new(db);
    let public_dir = std::env::current_dir().unwrap().join("public");

    println!("\n\n🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .service(actix_files::Files::new("/v1/images", &public_dir))
            .configure(routes::init)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
