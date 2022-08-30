use super::Pro;
use rocket::State;
use sqlx::{Pool, Sqlite};

pub async fn list_by_team_id(pool: &State<Pool<Sqlite>>, team_id: i64) -> Vec<Pro> {
    sqlx::query_as!(Pro, "SELECT * FROM pro WHERE team_id = ?", team_id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default()
}
