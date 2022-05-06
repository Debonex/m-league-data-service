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
            routes![routes::season_year::season_year_list],
        )
        .mount("/season", routes![routes::season::get_season_list])
        .mount("/team", routes![routes::team::get_team_list])
        .mount(
            "/pro",
            routes![routes::pro::pro_list, routes::pro::pro_data],
        )
}
