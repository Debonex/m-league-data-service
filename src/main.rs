#[macro_use]
extern crate rocket;

mod common;
mod game;
mod pool;
mod pro;
mod season;
mod season_year;
mod team;

use dotenv::dotenv;
use sqlx::{Pool, Sqlite};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let pool = pool::get_pool().await.unwrap();

    rocket::build()
        .manage::<Pool<Sqlite>>(pool)
        .mount("/", routes![index])
        .mount(
            "/season_year",
            routes![season_year::routes::all, season_year::routes::statistic],
        )
        .mount(
            "/season",
            routes![
                season::routes::all,
                season::routes::list,
                season::routes::statistic
            ],
        )
        .mount(
            "/team",
            routes![
                team::routes::all,
                team::routes::info,
                team::routes::info_by_pro_id,
                team::routes::statistic
            ],
        )
        .mount(
            "/pro",
            routes![
                pro::routes::all,
                pro::routes::statistic,
                pro::routes::rank,
                pro::routes::info,
                pro::routes::list_by_team_id
            ],
        )
        .mount(
            "/game",
            routes![
                game::routes::history_pro_pro,
                game::routes::history_pro_team,
                game::routes::history_team_team
            ],
        )
}
