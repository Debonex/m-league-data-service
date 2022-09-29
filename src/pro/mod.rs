pub mod routes;
pub mod services;

use crate::common::ranks::{Value, ValueItem};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Pro {
    pub id: i64,
    pub pro_name: Option<String>,
    pub team_id: Option<i64>,
    pub birth: Option<String>,
    pub birth_place: Option<String>,
    pub org: Option<String>,
    pub pro_year: Option<i64>,
}

impl Default for Pro {
    fn default() -> Self {
        Pro {
            id: -1,
            pro_name: None,
            team_id: None,
            birth: None,
            birth_place: None,
            org: None,
            pro_year: None,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProValueItem {
    pub pro_id: i64,
    pub pro_name: String,
    pub team_id: i64,
    pub value: Value,
}

impl ValueItem for ProValueItem {
    fn get_value(self) -> Value {
        self.value
    }

    fn set_value(&mut self, value: Value) {
        self.value = value
    }
}
