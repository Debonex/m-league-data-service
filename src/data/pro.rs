use super::*;

pub async fn get_pro_list(conn: Connection<'_, Db>) -> Vec<ProModel> {
    let db = conn.into_inner();
    ProEntity::find().all(db).await.unwrap_or(vec![])
}
