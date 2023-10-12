
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer, web};
use actix_web::{HttpResponse, Responder};
use serde_json::json;

mod config;
mod guards;
mod logger;
mod models;
mod routes;

use routes::{google_oauth_handler, users_me, logout};

use models::AppState;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/v1")
      .service(health_checker_handler)
      .service(google_oauth_handler)
      .service(logout)
      .service(users_me);
  
    conf.service(scope);
}

#[get("/ping")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success", "message": "Pong :D"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();

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
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
