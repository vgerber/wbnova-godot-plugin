use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateNovaVersionRequestBodyJson {
    pub channel: String,
}
