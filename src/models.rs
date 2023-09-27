use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, AsChangeset, prelude::{Associations, Identifiable}};
use rocket::serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name=crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>
}

#[derive(Queryable, Debug, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Debug)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32
}

#[derive(Insertable, )]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32
}