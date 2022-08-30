use super::Team;
use crate::{
    common::{
        season_pro,
        statistics::{self, Statistic},
    },
    pro,
};
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};
use sqlx::{Pool, Sqlite};

#[get("/all")]
pub async fn all(pool: &State<Pool<Sqlite>>) -> Json<Vec<Team>> {
    let teams = sqlx::query_as!(Team, "select * from team")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();

    Json(teams)
}

#[get("/info/<team_id>")]
pub async fn info(pool: &State<Pool<Sqlite>>, team_id: i64) -> Json<Team> {
    let team = sqlx::query_as!(Team, "select * from team where id = ?", team_id)
        .fetch_one(pool.inner())
        .await
        .unwrap_or_default();

    Json(team)
}

#[get("/info_by_pro_id/<pro_id>")]
pub async fn info_by_pro_id(pool: &State<Pool<Sqlite>>, pro_id: i64) -> Json<Team> {
    let team = sqlx::query_as!(
        Team,
        "select team.id as id, team_name, team_code from team left join pro on team.id = pro.team_id where pro.id = ?",
        pro_id
    )
    .fetch_one(pool.inner())
    .await
    .unwrap_or_default();

    Json(team)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TeamStatisticParams {
    team_id: i64,
    seasons: Option<Vec<i64>>,
}

#[post("/statistic", format = "json", data = "<params>")]
pub async fn statistic(
    pool: &State<Pool<Sqlite>>,
    params: Json<TeamStatisticParams>,
) -> Json<Statistic> {
    let pros = pro::services::list_by_team_id(pool, params.team_id).await;
    let sp_list = season_pro::select_season_pro(
        pool,
        &Some(pros.iter().map(|pro| pro.id).collect()),
        &params.seasons,
    )
    .await;
    Json(statistics::statistics(sp_list))
}
