mod controllers;
mod database;
mod dto;
mod models;
mod utils;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{
    web::{get, Data},
    App, HttpResponse, HttpServer,
};

use serde_json::json;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status":"success",
        "message":"Server is running"
    }))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = database::connect_cluster().await.unwrap();
    let db_data = Data::new(database);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .app_data(db_data.clone())
            .route("/", get().to(health_check))
            .wrap(cors)
            .wrap(Logger::new("%a %{User Agent}i %r %s %D"));
        return app;
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
