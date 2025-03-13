use crate::objects::io_value::IoValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PauseOnIo {
    pub comparator: String,
    pub io: IoValue,
}
