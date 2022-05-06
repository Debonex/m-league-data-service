use super::*;
use crate::domain::season_year as SeasonYearDomain;
use crate::entity::season_year::Model as SeasonYearModel;

#[get("/list")]
pub async fn season_year_list(conn: Connection<'_, Db>) -> Json<Vec<SeasonYearModel>> {
    let db = conn.into_inner();
    Json(SeasonYearDomain::season_year_list(db).await)
}
