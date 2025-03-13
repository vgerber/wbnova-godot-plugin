use crate::objects::initialize_movement_request::InitializeMovementRequest;
use crate::objects::pause_movement_request::PauseMovementRequest;
use crate::objects::playback_speed_request::PlaybackSpeedRequest;
use crate::objects::start_movement_request::StartMovementRequest;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExecuteTrajectoryRequestBodyJson {
    StartMovementRequestValue(StartMovementRequest),
    PauseMovementRequestValue(PauseMovementRequest),
    PlaybackSpeedRequestValue(PlaybackSpeedRequest),
    InitializeMovementRequestValue(InitializeMovementRequest),
}
