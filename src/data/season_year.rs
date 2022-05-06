use super::*;

pub async fn get_season_year_list(db: &DatabaseConnection) -> Vec<SeasonYearModel> {
    SeasonYearEntity::find().all(db).await.unwrap_or(vec![])
}
