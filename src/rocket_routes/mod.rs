use rocket::response::status::Custom;
use serde_json::{Value, json};
use rocket::http::Status;



pub mod rustaceans;
pub mod crates;

pub fn server_error(e: Box< dyn std::error::Error>) -> Custom<Value>{
        log::error!("{}", e);
        Custom(Status::InternalServerError, json!("Error"))
}