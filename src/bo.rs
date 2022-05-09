use serde::Serialize;

pub struct ProTupleValueItem {
    pub pro_id: i32,
    pub pro_name: String,
    pub values: (Value, Value),
}

#[derive(PartialEq, PartialOrd)]
pub enum Value {
    Integer(i32),
    Float(f64),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Integer(v) => serializer.serialize_i32(*v),
            Value::Float(v) => serializer.serialize_f64(*v),
        }
    }
}

impl Value {
    pub fn float_value(&self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn integer_value(&self) -> Option<i32> {
        match self {
            Value::Integer(v) => Some(*v),
            _ => None,
        }
    }
}
