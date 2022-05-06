use super::*;

/// 获取所有队伍的基本信息
pub async fn select_all_team(db: &DatabaseConnection) -> Vec<TeamModel> {
    TeamEntity::find().all(db).await.unwrap_or(vec![])
}
