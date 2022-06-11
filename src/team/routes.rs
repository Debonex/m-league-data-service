use super::Team;
use rocket::{serde::json::Json, State};
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
