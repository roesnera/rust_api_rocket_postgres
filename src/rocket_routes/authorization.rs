use rocket::serde::json::Json;
use serde_json::json;
use crate::auth::{Credentials, authorize_user};

use crate::repositories::UserRepository;

use super::{DbConn, server_error};



#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn) -> Result<serde_json::Value, rocket::response::status::Custom<serde_json::Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(c, &credentials.username)
        .map(|user| {
            if let Ok(session_id) = authorize_user(&user, &credentials)
            {
                return json!(session_id);
            }
            return json!("Unauthorized");
            
        })
        .map_err(|e| server_error(e.into()))
    }).await
}