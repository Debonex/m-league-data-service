use super::*;

/// 从db获取所有pro的基本信息
pub async fn select_all_pro(db: &DatabaseConnection) -> Vec<ProModel> {
    ProEntity::find().all(db).await.unwrap_or(vec![])
}
