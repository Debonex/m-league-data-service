use super::*;

/// 从db获取所有pro的基本信息
pub async fn select_all_pro(db: &DatabaseConnection) -> Vec<ProModel> {
    ProEntity::find().all(db).await.unwrap_or(vec![])
}

/// 从db根据id获取某个pro的基本信息
pub async fn select_pro(db: &DatabaseConnection, id: i32) -> Option<ProModel> {
    ProEntity::find_by_id(id).one(db).await.unwrap_or_default()
}
