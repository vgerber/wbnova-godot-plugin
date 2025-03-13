use crate::objects::google_protobuf_any::GoogleProtobufAny;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Status {
    pub details: Option<Vec<GoogleProtobufAny>>,
    pub code: Option<i32>,
    pub message: Option<String>,
}
