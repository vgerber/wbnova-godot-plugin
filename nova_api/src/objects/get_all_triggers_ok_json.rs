use crate::objects::trigger_object::TriggerObject;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetAllTriggersOkJson {
    pub triggers: Option<Vec<TriggerObject>>,
}
