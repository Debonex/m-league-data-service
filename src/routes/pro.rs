use super::*;
use crate::domain::pro as ProDomain;
use crate::entity::pro::Model as ProModel;
use crate::vo::{ProValueItem, Statistic};
use serde::Deserialize;

#[get("/all")]
pub async fn all(conn: Connection<'_, Db>) -> Json<Vec<ProModel>> {
    let db = conn.into_inner();
    Json(ProDomain::all(db).await)
}

#[derive(Deserialize)]
pub struct ProStatisticParams {
    id: i32,
    seasons: Option<Vec<i32>>,
}

#[post("/statistic", format = "json", data = "<params>")]
pub async fn statistic(
    conn: Connection<'_, Db>,
    params: Json<ProStatisticParams>,
) -> Json<Statistic> {
    let db = conn.into_inner();
    Json(ProDomain::statistic(db, params.id, &params.seasons).await)
}

#[derive(Deserialize)]
pub struct ProRankParams {
    key: String,
    seasons: Option<Vec<i32>>,
}

#[post("/rank", format = "json", data = "<params>")]
pub async fn rank(
    conn: Connection<'_, Db>,
    params: Json<ProRankParams>,
) -> Json<Vec<ProValueItem>> {
    let db = conn.into_inner();
    Json(ProDomain::rank(db, params.key.as_str(), &params.seasons).await)
}
