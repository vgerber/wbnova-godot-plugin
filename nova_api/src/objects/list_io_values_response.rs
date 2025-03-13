use crate::objects::io_value::IoValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListIoValuesResponse {
    pub io_values: Vec<IoValue>,
}
