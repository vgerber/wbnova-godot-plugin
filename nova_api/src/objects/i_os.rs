use crate::objects::io::Io;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct IOs {
    #[serde(alias = "IOs")]
    pub i_os: Vec<Io>,
}
