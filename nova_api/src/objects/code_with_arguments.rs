use crate::objects::initial_state::InitialState;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CodeWithArguments {
    pub code: String,
    pub initial_state: Option<InitialState>,
}
