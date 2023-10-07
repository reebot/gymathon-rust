use actix_web::{web, HttpResponse, Error, error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use r2d2::PooledConnection;
use crate::model::Booking;
use crate::errors::MyError;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn bookings_route(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn: PooledConnection<ConnectionManager<PgConnection>> = pool
        .get()
        .map_err(|_| error::ErrorInternalServerError("Error getting db connection"))?;

    let bookings = Booking::load_all(&conn).map_err(|err| MyError {
        message: format!("Diesel error: {}", err),
    })?;

    Ok(HttpResponse::Ok().json(bookings))
}
