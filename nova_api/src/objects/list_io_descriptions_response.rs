use crate::objects::io_description::IoDescription;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListIoDescriptionsResponse {
    pub io_descriptions: Vec<IoDescription>,
}
