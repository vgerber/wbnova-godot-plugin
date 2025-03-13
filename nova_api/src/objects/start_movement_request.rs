use crate::objects::pause_on_io::PauseOnIo;
use crate::objects::set_io::SetIo;
use crate::objects::start_on_io::StartOnIo;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StartMovementRequest {
    pub set_ios: Option<Vec<SetIo>>,
    pub start_on_io: Option<StartOnIo>,
    pub message_type: Option<String>,
    pub direction: Option<String>,
    pub pause_on_io: Option<PauseOnIo>,
}
