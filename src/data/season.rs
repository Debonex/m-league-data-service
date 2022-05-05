use super::*;

pub async fn get_season_list(conn: Connection<'_, Db>) -> Vec<SeasonModel> {
    let db = conn.into_inner();
    SeasonEntity::find().all(db).await.unwrap_or(vec![])
}
