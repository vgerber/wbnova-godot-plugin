use crate::objects::pause_on_io::PauseOnIo;
use crate::objects::set_io::SetIo;
use crate::objects::start_on_io::StartOnIo;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MoveRequest {
    pub response_coordinate_system: Option<String>,
    pub pause_on_io: Option<PauseOnIo>,
    pub motion: String,
    pub start_on_io: Option<StartOnIo>,
    pub playback_speed_in_percent: i32,
    pub response_rate: Option<i32>,
    pub start_location_on_trajectory: Option<f64>,
    pub set_ios: Option<Vec<SetIo>>,
}
