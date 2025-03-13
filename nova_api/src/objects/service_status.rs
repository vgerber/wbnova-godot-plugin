use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ServiceStatus {
    pub service: String,
    pub status: Object,
}
