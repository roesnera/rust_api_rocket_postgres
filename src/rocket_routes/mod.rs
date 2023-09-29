use rocket::response::status::Custom;
use serde_json::{Value, json};
use rocket::http::Status;


pub mod authorization;
pub mod rustaceans;
pub mod crates;
use diesel::PgConnection;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub fn server_error(e: Box< dyn std::error::Error>) -> Custom<Value>{
        log::error!("{}", e);
        Custom(Status::InternalServerError, json!("Error"))
}

pub fn not_found_error(e: Box< dyn std::error::Error>) -> Custom<Value> {
        log::error!("{}", e);
        Custom(Status::NotFound, json!("Error: not found!"))
}