use super::*;
use crate::domain::team as TeamDomain;
use crate::entity::team::Model as TeamModel;

#[get("/list")]
pub async fn get_team_list(conn: Connection<'_, Db>) -> Json<Vec<TeamModel>> {
    let db = conn.into_inner();
    Json(TeamDomain::team_list(db).await)
}
