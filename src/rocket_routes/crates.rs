use diesel::result::Error;
use rocket::{serde::json::{Json, Value, serde_json::json}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{NewCrate,Crate, User}, repositories::CrateRepository, rocket_routes::EditorUser};
use crate::rocket_routes::DbConn;

use super::{server_error, not_found_error};


#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
        .map(|new_crate| json!(new_crate))
        .map_err(|e: Error| {
            if e == diesel::result::Error::NotFound {
                return not_found_error(e.into())
            }
            server_error(e.into())
        })
    }).await
}
#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(new_crate: Json<NewCrate>, db: DbConn, _user: EditorUser) -> Result<Custom<Value>, Custom<Value>> {
    println!("New crate received: {:?}", new_crate);
    db.run(move |c| {
        CrateRepository::create(c, new_crate.into_inner())
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|e| server_error(e.into()))
    }).await
}
#[rocket::put("/crates/<id>", format="json", data="<da_crate>")]
pub async fn update_crate(id: i32, da_crate: Json<Crate>, db: DbConn, _user: EditorUser) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, da_crate.into_inner())
        .map(|a_crate| json!(a_crate))
        .map_err(|e: Error| {
            if e == diesel::result::Error::NotFound {
                return not_found_error(e.into())
            }
            server_error(e.into())
        })
    }).await
}
#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn, _user: EditorUser) -> Result<NoContent, Custom<Value>> {
    /* 
        here we have to move the id into the callback because it may run in a parallel thread or otherwise outlive the containing function block
        this is not allowed in rust, so we need to specify that the variable will move into the callback and die out here
     */
    db.run(move |c| {
        CrateRepository::delete(c, id)
        .map(|_| NoContent)
        .map_err(|e: Error| {
            if e == diesel::result::Error::NotFound {
                return not_found_error(e.into())
            }
            server_error(e.into())
        })
    }).await
}