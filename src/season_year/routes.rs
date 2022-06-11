use super::SeasonYear;
use crate::{
    common::{
        season_pro::select_season_pro,
        statistics::{statistics, Statistic},
    },
    season::Season,
};
use rocket::{serde::json::Json, State};
use sqlx::{Pool, Sqlite};

#[get("/all")]
pub async fn all(pool: &State<Pool<Sqlite>>) -> Json<Vec<SeasonYear>> {
    let season_years = sqlx::query_as!(SeasonYear, "select * from season_year")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();

    Json(season_years)
}

#[get("/statistic/<id>")]
pub async fn statistic(pool: &State<Pool<Sqlite>>, id: i64) -> Json<Statistic> {
    let season_list = sqlx::query_as!(Season, "select * from season where season_year_id = ?", id)
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();

    let season_id_list = season_list.iter().map(|season| season.id).collect();

    let sp_list = select_season_pro(pool, &None, &Some(season_id_list)).await;

    Json(statistics(sp_list))
}
