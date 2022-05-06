use super::*;
use crate::domain::season as SeasonDomain;
use crate::entity::season::Model as SeasonModel;

#[get("/list")]
pub async fn get_season_list(conn: Connection<'_, Db>) -> Json<Vec<SeasonModel>> {
    let db = conn.into_inner();
    Json(SeasonDomain::season_list(db).await)
}
