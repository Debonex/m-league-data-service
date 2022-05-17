use super::*;
use crate::domain::team as TeamDomain;
use crate::entity::team::Model as TeamModel;

#[get("/all")]
pub async fn all(conn: Connection<'_, Db>) -> Json<Vec<TeamModel>> {
    let db = conn.into_inner();
    Json(TeamDomain::all(db).await)
}

#[get("/info/<team_id>")]
pub async fn info(conn: Connection<'_, Db>, team_id: i32) -> Json<TeamModel> {
    let db = conn.into_inner();
    Json(TeamDomain::info(db, team_id).await)
}

#[get("/info_by_pro_id/<pro_id>")]
pub async fn info_by_pro_id(conn: Connection<'_, Db>, pro_id: i32) -> Json<TeamModel> {
    let db = conn.into_inner();
    Json(TeamDomain::info_by_pro_id(db, pro_id).await)
}
