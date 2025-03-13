use crate::objects::move_request::MoveRequest;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamMoveForward {
    pub forward: MoveRequest,
}
