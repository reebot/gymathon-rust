use chrono::{NaiveDate, NaiveTime};
use diesel::{prelude::*, Insertable, Queryable};
use serde::Serialize;

use crate::schema::bookings;

#[derive(Debug, Queryable, Insertable, Serialize)]
#[table_name = "bookings"]
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
    pub description: Option<String>, // Note the use of Option for a nullable field
}

impl Booking {
    pub fn load_all(connection: &PgConnection) -> QueryResult<Vec<Booking>> {
        let result = bookings::dsl::bookings.load(connection);
        result
    }
}
