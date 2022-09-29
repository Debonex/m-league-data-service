pub mod routes;

use crate::common::ranks::{Value, ValueItem};
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TeamValueItem {
    pub team_id: i64,
    pub team_name: String,
    pub value: Value,
}

impl ValueItem for TeamValueItem {
    fn get_value(self) -> Value {
        self.value
    }

    fn set_value(&mut self, value: Value) {
        self.value = value
    }
}
