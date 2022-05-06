use super::*;
use crate::domain::pro as ProDomain;
use crate::entity::pro::Model as ProModel;
use crate::vo::Statistic;
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
