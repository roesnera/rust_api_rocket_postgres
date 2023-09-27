use diesel::{PgConnection, Connection};

use crate::{models::NewUser, repositories::{UserRepository, RoleRepository}};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load db url from env");
    PgConnection::establish(&database_url)
        .expect("Cannot connect to postgres")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let new_user = NewUser {username, password};
    let mut c = load_db_connection();
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    println!("Role assigned {:?}", roles);
}

pub fn list_users() {

}

pub fn delete_user(id: i32) {

}