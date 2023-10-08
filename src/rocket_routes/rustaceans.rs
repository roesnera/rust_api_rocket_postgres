use diesel::result::Error;
use rocket::{serde::json::{Json, Value, serde_json::json}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{NewRustacean,Rustacean, User}, repositories::RustaceanRepository};
use crate::rocket_routes::DbConn;

use super::{server_error, not_found_error};

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn, user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, db: DbConn, user: User ) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
        .map(|rustacean| json!(rustacean))
        .map_err(|e: Error| {
            if e == diesel::result::Error::NotFound {
                return not_found_error(e.into())
            }
            server_error(e.into())
        })
    }).await
}
#[rocket::post("/rustaceans", format="json", data="<new_rustacean>")]
pub async fn create_rustacean(new_rustacean: Json<NewRustacean>, db: DbConn, user: User) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::put("/rustaceans/<id>", format="json", data="<rustacean>")]
pub async fn update_rustacean(id: i32, rustacean: Json<Rustacean>, db: DbConn, user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::update(c, id, rustacean.into_inner())
        .map(|rustacean| json!(rustacean))
        .map_err(|e: Error| {
            if e == diesel::result::Error::NotFound {
                return not_found_error(e.into())
            }
            server_error(e.into())
        })
    }).await
}
#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConn, user: User) -> Result<NoContent, Custom<Value>> {
    /* 
        here we have to move the id into the callback because it may run in a parallel thread or otherwise outlive the containing function block
        this is not allowed in rust, so we need to specify that the variable will move into the callback and die out here
     */
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
        .map(|_| NoContent)
        .map_err(|e: Error| {
            if e == diesel::result::Error::NotFound {
                return not_found_error(e.into())
            }
            server_error(e.into())
        })
    }).await
}