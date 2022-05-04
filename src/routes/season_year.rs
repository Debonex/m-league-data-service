use rocket::serde::json::Json;
use sea_orm::EntityTrait;

use crate::{
    entity::season_year::{Entity, Model},
    pool::Db,
};

use sea_orm_rocket::Connection;

#[get("/")]
pub async fn all(conn: Connection<'_, Db>) -> Json<Vec<Model>> {
    let db = conn.into_inner();
    let result = Entity::find().all(db).await;
    match result {
        Ok(list) => Json(list),
        Err(_) => Json(vec![]),
    }
}
