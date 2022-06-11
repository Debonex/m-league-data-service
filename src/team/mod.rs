pub mod routes;

use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    pub id: i64,
    pub team_name: Option<String>,
    pub team_code: Option<String>,
}

impl Default for Team {
    fn default() -> Self {
        Team {
            id: -1,
            team_name: None,
            team_code: None,
        }
    }
}
