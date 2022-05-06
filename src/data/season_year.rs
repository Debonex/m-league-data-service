use super::*;

/// 从db获取所有赛季年的基本信息
pub async fn select_all_season_year(db: &DatabaseConnection) -> Vec<SeasonYearModel> {
    SeasonYearEntity::find().all(db).await.unwrap_or(vec![])
}
