use crate::common::statistics::Statistic;
use crate::common::{season_pro, statistics};
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};
use sqlx::{Pool, Sqlite};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LeagueStatisticParams {
    teams: Option<Vec<i64>>,
    seasons: Option<Vec<i64>>,
}

#[post("/statistic", format = "json", data = "<params>")]
pub async fn statistic(
    pool: &State<Pool<Sqlite>>,
    params: Json<LeagueStatisticParams>,
) -> Json<Statistic> {
    let sp_list =
        season_pro::select_season_pro_by_teams(pool, &params.teams, &params.seasons).await;
    Json(statistics::statistics(sp_list))
}
