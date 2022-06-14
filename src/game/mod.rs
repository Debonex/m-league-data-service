pub mod routes;

use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct HistoryResult {
    pub point: f32,
    pub games: Vec<Game>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub id: i64,
    pub season_id: i64,
    pub time: String,
    pub pros: Vec<GamePro>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GamePro {
    pub id: i64,
    pub pro_name: String,
    pub team_id: i64,
    pub point: f32,
}
