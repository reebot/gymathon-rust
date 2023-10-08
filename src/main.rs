use warp::Filter;
use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use tokio;
use sqlx::FromRow;
use warp::filters::cors;


#[derive(Debug, Deserialize, Serialize, FromRow)] // Ensure to derive FromRow
pub struct Activity {
    pub id: String,
    #[serde(rename = "activityType")] // Use serde rename if the field name doesn't match the column name
    pub activity_type: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "seatBooking")]
    pub seat_booking: String,
    #[serde(rename = "waitingListCapacity")]
    pub waiting_list_capacity: i32,
    #[serde(rename = "centerId")]
    pub center_id: i32,
    #[serde(rename = "centerName")]
    pub center_name: String,
    #[serde(rename = "personCenterId")]
    pub person_center_id: i32,
    pub date: NaiveDate,
    #[serde(rename = "startTime")]
    pub start_time: NaiveTime,
    #[serde(rename = "endTime")]
    pub end_time: NaiveTime,
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: i32,
}
async fn get_activities(pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let rows = sqlx::query_as::<_, Activity>(
        r#"
        SELECT * FROM activity
        "#,
    )
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("db error: {}", e);
            warp::reject::not_found()
        })?;

    Ok(warp::reply::json(&rows))
}
/*async fn get_activities(pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
    let date = "2023-10-08".parse::<NaiveDate>().unwrap();
    let rows = sqlx::query_as::<_, Activity>(
        r#"
        SELECT * FROM activity WHERE date = $1
        "#,
    )
        .bind(date)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            eprintln!("db error: {}", e);
            warp::reject::not_found()
        })?;

    Ok(warp::reply::json(&rows))
}*/


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database and handle potential errors
    let pool = match PgPool::connect(&url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Error connecting to database: {}", e);
            std::process::exit(1);
        }
    };

    let cors = cors::cors()
        .allow_methods(vec!["GET", "POST", "DELETE"])
        .allow_headers(vec!["Authorization", "Content-Type"])
        .allow_any_origin();

    let activity_route = warp::path("activities")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(get_activities);

    // Use the `routes` variable here
    let routes = activity_route.with(cors).with(warp::log("api"));

    println!("Server started at http://127.0.0.1:8000"); // Log to know server has started
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}

fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
