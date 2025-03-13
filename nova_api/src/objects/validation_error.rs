use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub msg: String,
    #[serde(alias = "type")]
    pub type_name: String,
}
