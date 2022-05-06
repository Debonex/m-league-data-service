use super::*;

pub async fn get_season_list(db: &DatabaseConnection) -> Vec<SeasonModel> {
    SeasonEntity::find().all(db).await.unwrap_or(vec![])
}
