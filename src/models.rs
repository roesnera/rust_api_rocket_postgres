use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, AsChangeset};
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

#[derive(Queryable)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name=crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub name: String,
    pub code: String,
    pub version: String,
    pub description: Option<String>
}