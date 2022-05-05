use super::*;

pub async fn get_season_year_list(conn: Connection<'_, Db>) -> Vec<SeasonYearModel> {
    let db = conn.into_inner();
    SeasonYearEntity::find().all(db).await.unwrap_or(vec![])
}
