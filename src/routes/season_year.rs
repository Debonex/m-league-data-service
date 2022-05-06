use super::*;
use crate::domain::season_year as SeasonYearDomain;
use crate::entity::season_year::Model as SeasonYearModel;
use crate::vo::Statistic;

#[get("/all")]
pub async fn all(conn: Connection<'_, Db>) -> Json<Vec<SeasonYearModel>> {
    let db = conn.into_inner();
    Json(SeasonYearDomain::all(db).await)
}

#[get("/statistic/<id>")]
pub async fn statistic(conn: Connection<'_, Db>, id: i32) -> Json<Statistic> {
    let db = conn.into_inner();
    Json(SeasonYearDomain::statistic(db, id).await)
}
