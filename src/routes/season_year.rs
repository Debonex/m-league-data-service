use super::*;
use crate::data::season_year as SeasonYearDao;
use crate::entity::season_year::Model;

#[get("/list")]
pub async fn season_year_list(conn: Connection<'_, Db>) -> Json<Vec<Model>> {
    Json(SeasonYearDao::get_season_year_list(conn).await)
}
