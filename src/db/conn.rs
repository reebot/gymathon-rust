use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

#[derive(Clone)]
struct PgConfig {
    url: String,
}

impl PgConfig {

    fn from_env() -> Result<Self,dotenv::Error> {
        let url = env::var("DATABASE_URL")?;
        Ok(Self { url })
    }

}

pub async fn create_pool(config: &PgConfig) -> Result<Pool, deadpool_postgres::Error> {

    let mgr = PostgresConnectionManager::new(config.url.as_str(), NoTls);

    Pool::builder(mgr)
        .max_size(5)
        .build()
}

pub async fn get_conn<'a>(pool: &'a Pool<PostgresConnectionManager>)
                          -> Result<PooledConnection<'a>, deadpool_postgres::Error> {

    pool.get().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let config = PgConfig::from_env()?;
    let pool = create_pool(&config).await?;

    let conn = get_conn(&pool).await?;

    // Use connection

    Ok(())
}