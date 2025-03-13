use crate::objects::initialize_movement_response::InitializeMovementResponse;
use crate::objects::movement::Movement;
use crate::objects::movement_error::MovementError;
use crate::objects::pause_movement_response::PauseMovementResponse;
use crate::objects::playback_speed_response::PlaybackSpeedResponse;
use crate::objects::standstill::Standstill;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ExecuteTrajectoryResponse {
    InitializeMovementResponseValue(InitializeMovementResponse),
    MovementValue(Movement),
    StandstillValue(Standstill),
    PauseMovementResponseValue(PauseMovementResponse),
    PlaybackSpeedResponseValue(PlaybackSpeedResponse),
    MovementErrorValue(MovementError),
}
