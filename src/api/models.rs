use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime};


#[derive(Serialize, Deserialize, Debug)]
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

// pub struct Booking {
//     pub id: Option<String>,
//     pub center_id: Option<i32>,
//     pub name: Option<String>,
//     pub booking_date: Option<NaiveDate>,
//     pub start_time: Option<NaiveTime>,
//     pub end_time: Option<NaiveTime>,
//     pub duration_minutes: Option<i32>,
//     pub activity_id: Option<String>,
//     pub booking_status: Option<String>,
//     pub provider: Option<String>,
//     pub room_name: Option<String>,
//     pub center_name: Option<String>,
//     pub description: Option<String>,
// }