use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GoogleProtobufAny {
    #[serde(alias = "@type")]
    pub type_name: Option<String>,
}
