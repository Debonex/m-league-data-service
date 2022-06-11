pub mod routes;

use rocket::serde::{Serialize, Serializer};

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

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Value {
    Integer(i64),
    Float(f32),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::Integer(v) => serializer.serialize_i64(*v),
            Value::Float(v) => serializer.serialize_f32(*v),
        }
    }
}

impl Value {
    pub fn float_value(&self) -> Option<f32> {
        match self {
            Value::Float(v) => Some(*v),
            _ => None,
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
