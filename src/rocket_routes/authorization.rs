use argon2::{PasswordHash, PasswordVerifier};
use rocket::serde::json::Json;
use serde_json::json;

use crate::repositories::UserRepository;

use super::{DbConn, server_error};


#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn) -> Result<serde_json::Value, rocket::response::status::Custom<serde_json::Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(c, &credentials.username)
        .map(|user| {
            let db_hash = PasswordHash::new(&user.password).unwrap();
            let argon = argon2::Argon2::default();
            if argon.verify_password(credentials.password.as_bytes(), &db_hash).is_ok() {
                return json!("Success");
            } else {
                return json!("Error");
            }
        })
        .map_err(|e| server_error(e.into()))
    }).await
}