use crate::objects::payload::Payload;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListPayloadsResponse {
    pub payloads: Option<Vec<Payload>>,
}
