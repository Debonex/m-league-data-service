use super::*;
use crate::data::team as TeamDao;
use crate::entity::team::Model as TeamModel;

#[get("/list")]
pub async fn get_team_list(conn: Connection<'_, Db>) -> Json<Vec<TeamModel>> {
    Json(TeamDao::get_team_list(conn).await)
}
