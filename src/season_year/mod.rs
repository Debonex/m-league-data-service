pub mod routes;

use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SeasonYear {
    pub id: i64,
    pub season_year_name: Option<String>,
}
