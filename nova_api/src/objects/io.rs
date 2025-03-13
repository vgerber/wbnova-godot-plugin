use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Io {
    pub bool: Option<bool>,
    pub double: Option<f64>,
    pub integer: Option<String>,
    pub name: String,
    pub direction: String,
}
