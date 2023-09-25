mod models;
mod schema;
mod repositories;
mod rocket_routes;
use diesel::PgConnection;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            rocket_routes::rustaceans::get_rustaceans,
            rocket_routes::rustaceans::view_rustacean,
            rocket_routes::rustaceans::create_rustacean,
            rocket_routes::rustaceans::update_rustacean,
            rocket_routes::rustaceans::delete_rustacean,
            rocket_routes::crates::get_crates,
            rocket_routes::crates::view_crate,
            rocket_routes::crates::create_crate,
            rocket_routes::crates::update_crate,
            rocket_routes::crates::delete_crate,
            
        ])
        /* 
            fairing makes sure that your dependencies are loaded up properly before launchhing
            otherwise, panic
         */
        .attach(DbConn::fairing())
        .launch()
        .await;
}
