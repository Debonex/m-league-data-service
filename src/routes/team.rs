use super::*;
use crate::domain::team as TeamDomain;
use crate::entity::team::Model as TeamModel;

#[get("/all")]
pub async fn all(conn: Connection<'_, Db>) -> Json<Vec<TeamModel>> {
    let db = conn.into_inner();
    Json(TeamDomain::all(db).await)
}
