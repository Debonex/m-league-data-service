pub mod routes;

use rocket::serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Season {
    pub id: i64,
    pub season_year_id: Option<i64>,
    pub season_name: Option<String>,
    pub season_type: Option<String>,
    pub season_code: Option<String>,
}
