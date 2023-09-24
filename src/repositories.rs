use diesel::{PgConnection, QueryResult};
use diesel::prelude::*;

use crate::models::*;
use crate::schema::*;


pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(c)
    }
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load(c)
    }
    pub fn create(c: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
    }
    pub fn update(c: &mut PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email)
            ))
            .get_result(c)
    }
    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(c)
    }
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).load(c)
    }
    pub fn create(c: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(c)
    }
    pub fn update(c: &mut PgConnection, id: i32, crate_to_add: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(crate_to_add.rustacean_id),
                crates::name.eq(crate_to_add.name),
                crates::code.eq(crate_to_add.code),
                crates::version.eq(crate_to_add.version),
                crates::description.eq(crate_to_add.description),
            ))
            .get_result(c)
    }
    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c)
    }
}