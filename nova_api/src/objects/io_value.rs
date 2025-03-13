use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IoValue {
    pub integer_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub floating_value: Option<f64>,
    pub io: String,
}
