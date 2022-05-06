use super::*;
use crate::domain::season as SeasonDomain;
use crate::entity::season::Model as SeasonModel;
use crate::vo::Statistic;
use serde::Deserialize;

#[get("/all")]
pub async fn all(conn: Connection<'_, Db>) -> Json<Vec<SeasonModel>> {
    let db = conn.into_inner();
    Json(SeasonDomain::all_season_list(db).await)
}

#[derive(Deserialize)]
pub struct SeasonListParams {
    season_year_list: Option<Vec<i32>>,
    season_type: Option<String>,
}

#[post("/list", format = "json", data = "<params>")]
pub async fn list(
    conn: Connection<'_, Db>,
    params: Json<SeasonListParams>,
) -> Json<Vec<SeasonModel>> {
    let db = conn.into_inner();
    Json(SeasonDomain::season_list(db, &params.season_year_list, &params.season_type).await)
}

#[derive(Deserialize)]
pub struct SeasonStatisticParams {
    seasons: Option<Vec<i32>>,
}

#[post("/statistic", format = "json", data = "<params>")]
pub async fn statistics(
    conn: Connection<'_, Db>,
    params: Json<SeasonStatisticParams>,
) -> Json<Statistic> {
    let db = conn.into_inner();
    Json(SeasonDomain::statistic_by_id(db, &params.seasons).await)
}
