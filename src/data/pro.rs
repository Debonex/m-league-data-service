use super::*;

/// 从db获取所有选手的基本信息
pub async fn select_all_pro(db: &DatabaseConnection) -> Vec<ProModel> {
    ProEntity::find().all(db).await.unwrap_or(vec![])
}

/// 从db根据id获取某个选手的基本信息
pub async fn select_pro(db: &DatabaseConnection, id: i32) -> Option<ProModel> {
    ProEntity::find_by_id(id).one(db).await.unwrap_or_default()
}

/// 从db根据队伍id获取该队伍所有选手的基本信息
pub async fn select_pro_by_team_id(db: &DatabaseConnection, team_id: i32) -> Vec<ProModel> {
    ProEntity::find()
        .filter(ProColumn::TeamId.eq(team_id))
        .all(db)
        .await
        .unwrap_or(vec![])
}
