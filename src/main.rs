use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;
use tokio;

#[derive(Debug, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activity_type: String,
    pub name: String,
    pub description: String,
    pub seat_booking: String,
    pub waiting_list_capacity: i32,
    pub center_id: i32,
    pub center_name: String,
    pub person_center_id: i32,
    pub date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub duration_minutes: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&url).await?;

    // Parse the string into a NaiveDate object
    let date = "2023-10-08".parse::<NaiveDate>()?;

    // Query to fetch all activities on a specific date
    let rows = sqlx::query!(
        r#"
        SELECT * FROM activity WHERE date = $1
        "#,
        date
    )
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!(
            "Activity Name: {}, Description: {:?}, Start Time: {}",
            row.name, row.description, row.start_time
        );
    }

    Ok(())
}
