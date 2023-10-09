// use rocket::{response::status::Custom, Request, request::{self, FromRequest, Outcome}};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket_db_pools::{deadpool_redis::{self, redis::AsyncCommands}, Database, Connection};


pub mod authorization;
pub mod rustaceans;
pub mod crates;
use diesel::PgConnection;

use crate::models::RoleCode;
use crate::repositories::RoleRepository;
use crate::{models::User, repositories::UserRepository};

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

pub fn server_error(e: Box< dyn std::error::Error>) -> Custom<Value>{
        log::error!("{}", e);
        Custom(Status::InternalServerError, json!("Error"))
}

pub fn not_found_error(e: Box< dyn std::error::Error>) -> Custom<Value> {
        log::error!("{}", e);
        Custom(Status::NotFound, json!("Error: not found!"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
        type Error = ();
        async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                // Authorization: Bearer SESSION_ID_128_CHARS_LONG
                let session_header = request.headers().get_one("Authorization")
                        .map(|v| v.split_whitespace().collect::<Vec<_>>())
                        .filter(|v| v.len() == 2 && v[0] == "Bearer");

                if let Some(session_value) = session_header {
                        let mut cache: Connection<CacheConn> = request.guard::<Connection<CacheConn>>().await
                                .expect("Cannot connect to redis in request guard");

                        let db: DbConn = request.guard::<DbConn>().await
                                .expect("Cannot connect to postres in request guard");

                        let result = cache.get::<_, i32>(format!("sessions/{}", session_value[1])).await;

                        if let Ok(user_id) = result {
                                return match db.run(move |c| UserRepository::find(c, user_id)).await {
                                        Ok(user) => Outcome::Success(user),
                                        _ => Outcome::Failure((Status::Unauthorized, ()))
                                }
                        }
                }

                Outcome::Failure((Status::Unauthorized, ()))

        }
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
        type Error = ();
        async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                let user = request.guard::<User>().await.expect("Cannot retrieve logged in user in request guard");

                let db: DbConn = request.guard::<DbConn>().await
                        .expect("Cannot connect to postres in request guard");

                let editor_option = db.run(|c| {
                        match RoleRepository::find_by_user(c, &user) {
                                Ok(roles) => {
                                        log::info!("Assigned roles: {:?}", roles);
                                        let is_editor = roles.iter().any(|r| match r.code {
                                        RoleCode::Admin => true,
                                        RoleCode::Editor => true,
                                        _ => false,
                                });
                                log::info!("is_editor: {:?}", is_editor);
                                is_editor.then_some(EditorUser(user))
                                },
                                _ => None
                        }
                }).await;

                match editor_option {
                        Some(editor_user) => Outcome::Success(editor_user),
                        None => Outcome::Failure((Status::Unauthorized, ()))
                }

        }
}