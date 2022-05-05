use super::*;

pub async fn get_season_pro_by_pro_id(conn: Connection<'_, Db>, id: u32) -> Vec<SPModel> {
    let db = conn.into_inner();
    let result = SPEntity::find()
        .filter(SPColumn::ProId.eq(id))
        .all(db)
        .await;
    return result.unwrap_or(vec![]);
}
