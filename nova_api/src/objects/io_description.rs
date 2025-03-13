use crate::objects::io_value::IoValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IoDescription {
    pub max: Option<IoValue>,
    pub id: String,
    pub value_type: String,
    pub unit: Option<String>,
    pub group: Option<String>,
    pub min: Option<IoValue>,
    pub name: String,
    #[serde(alias = "type")]
    pub type_name: String,
    pub bit_size: i32,
}
