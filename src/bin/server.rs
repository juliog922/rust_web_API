extern crate cr8s;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            cr8s::routes::options,
            cr8s::routes::authorizathion::me,
            cr8s::routes::authorizathion::login,
            cr8s::routes::rustaceans::get_rustaceans,
            cr8s::routes::rustaceans::view_rustacean,
            cr8s::routes::rustaceans::create_rustacean,
            cr8s::routes::rustaceans::update_rustacean,
            cr8s::routes::rustaceans::delete_rustacean,
            cr8s::routes::crates::get_crates,
            cr8s::routes::crates::view_crate,
            cr8s::routes::crates::create_crate,
            cr8s::routes::crates::update_crate,
            cr8s::routes::crates::delete_crate
        ])
        .attach(cr8s::routes::Cors)
        .attach(cr8s::routes::CacheConn::init())
        .attach(cr8s::routes::DbConn::init())
        .launch()
        .await;
}
