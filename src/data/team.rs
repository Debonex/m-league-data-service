use super::*;

pub async fn get_team_list(conn: Connection<'_, Db>) -> Vec<TeamModel> {
    let db = conn.into_inner();
    TeamEntity::find().all(db).await.unwrap_or(vec![])
}
