use crate::objects::object_value::ObjectValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum InitialState {
    ObjectValue(ObjectValue),
}
