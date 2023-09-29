extern crate rust_database_for_api;


#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            rust_database_for_api::rocket_routes::authorization::login,
            rust_database_for_api::rocket_routes::rustaceans::get_rustaceans,
            rust_database_for_api::rocket_routes::rustaceans::view_rustacean,
            rust_database_for_api::rocket_routes::rustaceans::create_rustacean,
            rust_database_for_api::rocket_routes::rustaceans::update_rustacean,
            rust_database_for_api::rocket_routes::rustaceans::delete_rustacean,
            rust_database_for_api::rocket_routes::crates::get_crates,
            rust_database_for_api::rocket_routes::crates::view_crate,
            rust_database_for_api::rocket_routes::crates::create_crate,
            rust_database_for_api::rocket_routes::crates::update_crate,
            rust_database_for_api::rocket_routes::crates::delete_crate,
            
        ])
        /* 
            fairing makes sure that your dependencies are loaded up properly before launchhing
            otherwise, panic
         */
        .attach(rust_database_for_api::rocket_routes::DbConn::fairing())
        .launch()
        .await;
}