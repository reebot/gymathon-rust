use diesel::pg::PgConnection;
pub(crate) use diesel::r2d2::{self, ConnectionManager};
use std::env;
use diesel::Connection;

// src/database.rs

//use diesel::pg::PgConnection;

// src/database.rs

use dotenv::var;

pub fn get_connection() -> PgConnection {
    PgConnection::establish(&format!("postgresql://{}:{}@{}:{}/{}", var("PG_USER").unwrap(), var("PG_PASSWORD").unwrap(), var("PG_HOST").unwrap(), var("PG_PORT").unwrap(), var("PG_DATABASE").unwrap())).unwrap()
}


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

fn database_url() -> String {
    format!(
        "postgresql://{}:{}@{}:{}/{}",
        env::var("PG_USER").expect("PG_USER must be set"),
        env::var("PG_PASSWORD").expect("PG_PASSWORD must be set"),
        env::var("PG_HOST").expect("PG_HOST must be set"),
        env::var("PG_PORT").expect("PG_PORT must be set"),
        env::var("PG_DATABASE").expect("PG_DATABASE must be set")
    )
}
