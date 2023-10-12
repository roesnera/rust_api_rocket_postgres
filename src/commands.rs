use chrono::{Datelike, Utc};
use diesel::{PgConnection, Connection};
use lettre::{Transport, message::header::ContentType, SmtpTransport, transport::smtp::authentication::Credentials};
use std::str::FromStr;

use crate::{models::{NewUser, RoleCode}, repositories::{UserRepository, RoleRepository, CrateRepository}, auth::hash_password};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load db url from env");
    PgConnection::establish(&database_url)
        .expect("Cannot connect to postgres")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection();

    let password_hash = hash_password(password).unwrap();

    let new_user = NewUser {username, password: password_hash};
    let role_codes = role_codes.iter().map(|v| RoleCode::from_str(&v).unwrap()).collect();
    println!("User role codes: {:?}", role_codes);
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    println!("Role assigned {:?}", roles);
}

pub fn list_users() {
    let mut c = load_db_connection();

    let users = UserRepository::find_all_with_roles(&mut c).unwrap();
    for user in users {
        println!("User: {:?}", user);
    }
}

pub fn delete_user(id: i32) {
    let mut c = load_db_connection();

    let _ = UserRepository::delete(&mut c, id);
}

pub fn send_digest(to: String, hours_since: i32) {
    let mut c = load_db_connection();

    println!("Send_digest triggered with to: {}, hours since: {}", to, hours_since);

    let crates = CrateRepository::find_since(&mut c, hours_since).unwrap();

    // let crates = CrateRepository::find_multiple(&mut c, 1).unwrap();

    println!("crates: {:?}", crates);

    if crates.len() > 0 {
        println!("Sending the digest for {} crates", crates.len());
        let tera = get_template_engine();
        let year = Utc::now().year();
        let mut context = tera::Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);
        let html_body = tera.render("email/digest.html", &context).unwrap();
        let message = lettre::Message::builder()
            .subject("Crates Digest")
            .from("Crates <info@crates.com>".parse().unwrap())
            .to(to.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();
        
    let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load smtp host from env");
    let smtp_username = std::env::var("SMTP_USERNAME").expect("Cannot load smtp username from env");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("Cannot load smtp password from env");
    let credentials = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(credentials)
        .build();
    mailer.send(&message).unwrap();
    }
}

fn get_template_engine() -> tera::Tera {
    tera::Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Parsing errors: {}", e);
    })
}