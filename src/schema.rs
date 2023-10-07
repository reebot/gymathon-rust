// @generated automatically by Diesel CLI.

diesel::table! {
    bookings (id) {
        id -> Varchar,
        center_id -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        booking_date -> Nullable<Date>,
        start_time -> Nullable<Time>,
        end_time -> Nullable<Time>,
        duration_minutes -> Nullable<Int4>,
        activity_id -> Nullable<Varchar>,
        booking_status -> Nullable<Varchar>,
        provider -> Nullable<Varchar>,
        room_name -> Nullable<Varchar>,
        center_name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}
