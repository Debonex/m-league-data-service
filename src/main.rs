#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use sea_orm_rocket::Database;

mod dto;
mod entity;
mod pool;
mod routes;

use pool::Db;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index,])
        .mount("/season_year", routes![routes::season_year::all])
        .mount("/pro", routes![routes::pro::pro_data])
}
