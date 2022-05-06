use super::*;

pub async fn get_pro_list(db: &DatabaseConnection) -> Vec<ProModel> {
    ProEntity::find().all(db).await.unwrap_or(vec![])
}
