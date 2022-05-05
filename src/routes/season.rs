use super::*;
use crate::data::season as SeasonDao;
use crate::entity::season::Model as SeasonModel;

#[get("/list")]
pub async fn get_season_list(conn: Connection<'_, Db>) -> Json<Vec<SeasonModel>> {
    Json(SeasonDao::get_season_list(conn).await)
}
