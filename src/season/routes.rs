use super::Season;
use crate::common::{
    format_sql_vec,
    season_pro::select_season_pro,
    statistics::{statistics, Statistic},
};
use rocket::{
    serde::{json::Json, Deserialize},
    State,
};
use sqlx::{Pool, Sqlite};

#[get("/all")]
pub async fn all(pool: &State<Pool<Sqlite>>) -> Json<Vec<Season>> {
    let seasons = sqlx::query_as!(Season, "select * from season")
        .fetch_all(pool.inner())
        .await
        .unwrap_or_default();

    Json(seasons)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SeasonListParams {
    season_year_list: Option<Vec<i64>>,
    season_type: Option<String>,
}

#[post("/list", format = "json", data = "<params>")]
pub async fn list(pool: &State<Pool<Sqlite>>, params: Json<SeasonListParams>) -> Json<Vec<Season>> {
    let sql_string = if let Some(season_year_list) = &params.season_year_list {
        if let Some(season_type) = &params.season_type {
            format!(
                "select * from season where season_type = '{}' and season_year_id in {}",
                season_type,
                format_sql_vec(season_year_list)
            )
        } else {
            format!(
                "select * from season where season_year_id in {}",
                format_sql_vec(season_year_list)
            )
        }
    } else if let Some(season_type) = &params.season_type {
        format!("select * from season where season_type = '{}'", season_type)
    } else {
        "select * from season".to_string()
    };

    Json(
        sqlx::query_as::<_, Season>(&sql_string)
            .fetch_all(pool.inner())
            .await
            .unwrap_or_default(),
    )
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SeasonStatisticParams {
    seasons: Option<Vec<i64>>,
}

#[post("/statistic", format = "json", data = "<params>")]
pub async fn statistic(
    pool: &State<Pool<Sqlite>>,
    params: Json<SeasonStatisticParams>,
) -> Json<Statistic> {
    let sp_list = select_season_pro(pool, &None, &params.seasons).await;

    Json(statistics(sp_list))
}
