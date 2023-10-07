// Import necessary modules and crates
mod database;
mod model;
mod routes;
mod schema;
mod errors;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Setup logging
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Initialize database connection pool
    let pool = database::init_pool();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // Pass database pool to application
            .service(
                web::resource("/api/v1/bookings")
                    .route(web::get().to(routes::bookings_route)),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
