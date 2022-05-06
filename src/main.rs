#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use sea_orm_rocket::Database;

mod data;
mod domain;
mod entity;
mod entity_impl;
mod pool;
mod routes;
mod vo;

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
        .mount(
            "/season_year",
            routes![routes::season_year::all, routes::season_year::statistic],
        )
        .mount(
            "/season",
            routes![
                routes::season::all,
                routes::season::list,
                routes::season::statistics
            ],
        )
        .mount("/team", routes![routes::team::all])
        .mount("/pro", routes![routes::pro::all, routes::pro::statistic])
}
