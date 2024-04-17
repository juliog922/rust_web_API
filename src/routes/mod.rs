use std::error::Error;

pub mod rustaceans;
pub mod crates;

use rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::response::status::Custom;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value>{
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

pub fn not_found_error(e: Box<dyn Error>) -> Custom<Value>{
    rocket::error!("{}", e);
    Custom(Status::NotFound, json!("Not found"))
}