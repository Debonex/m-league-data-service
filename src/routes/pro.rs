use super::*;
use crate::domain::pro as ProDomain;
use crate::dto::Statistic;
use crate::entity::pro::Model as ProModel;

#[get("/list")]
pub async fn pro_list(conn: Connection<'_, Db>) -> Json<Vec<ProModel>> {
    let db = conn.into_inner();
    Json(ProDomain::pro_list(db).await)
}

#[get("/data/<id>")]
pub async fn pro_data(conn: Connection<'_, Db>, id: u32) -> Json<Statistic> {
    let db = conn.into_inner();
    Json(ProDomain::pro_data(db, id).await)
}
