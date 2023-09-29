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

pub struct UserRepository;
impl UserRepository {
    pub fn find_by_username(c: &mut PgConnection, name: &String) -> QueryResult<User> {
        users::table.filter(users::username.eq(name)).first(c)
    }

    pub fn create(c: &mut PgConnection, new_user: NewUser, role_codes: Vec<String>) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, role_code.to_owned()) {
                    NewUserRole {user_id: user.id, role_id: role.id}
                } else {
                    let new_role = NewRole {name: role_code.to_owned(), code: role_code.to_owned()};
                    let role = RoleRepository::create(c, new_role)?;
                    NewUserRole {user_id: user.id, role_id: role.id}
                }
            };
            
            let _ = diesel::insert_into(users_roles::table)
            .values(new_user_role)
            .get_result::<UserRole>(c)?;
        }

        Ok(user)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(
            users_roles::table.filter(users_roles::user_id.eq(id))
        ).execute(c)?;
        
        diesel::delete(
            users::table.find(id)
        ).execute(c)
    }

    pub fn find_all_with_roles(c: &mut PgConnection) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load(c)?;

        let result = users_roles::table.inner_join(roles::table)
            .load::<(UserRole, Role)>(c)?
            .grouped_by(&users);

        // this method .zip() is sick
        // it creates a new iterable out of two distinct iterables where the new iterable
        // contains a tuple of data from iter1 and iter2 that are "zipped" together into pairs
        Ok(users.into_iter().zip(result).collect())
    }
}

pub struct RoleRepository;
impl RoleRepository {
    pub fn find_by_code(c: &mut PgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c)
    }

    pub fn create(c: &mut PgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
    }

    pub fn find_by_ids(c: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).get_results(c)
    }

    pub fn find_by_user(c: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        // here we had to add Associations to UserRole struct and Identifiable to UserRole and User struct
        let user_roles = UserRole::belonging_to(&user).get_results(c)?;

        let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(c, role_ids)
    }
}