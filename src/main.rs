use sqlx::postgres::PgRow;
use sqlx::Connection;
use sqlx::Row;
use std::env;
use dotenv::dotenv;
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Booking {
    pub id: String,
    pub center_id: i32,
    pub name: String,
    pub booking_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub duration_minutes: i32,
    pub activity_id: String,
    pub booking_status: String,
    pub provider: String,
    pub room_name: String,
    pub center_name: String,
    pub description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = sqlx::PgConnection::connect(&url).await?;

    // Example query to fetch a booking by id
    let row: (String,) = sqlx::query_as("SELECT name FROM bookings WHERE id = $1")
        .bind("1") // replace with the id you're searching for
        .fetch_one(&mut conn)
        .await?;

    println!("Booking Name: {:?}", row.0);

    Ok(())
}
