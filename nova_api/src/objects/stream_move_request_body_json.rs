use crate::objects::stream_move_backward_value::StreamMoveBackwardValue;
use crate::objects::stream_move_forward_value::StreamMoveForwardValue;
use crate::objects::stream_move_playback_speed_value::StreamMovePlaybackSpeedValue;
use crate::objects::stream_move_to_trajectory_value::StreamMoveToTrajectoryValue;
use crate::objects::stream_stop_value::StreamStopValue;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StreamMoveRequestBodyJson {
    StreamMoveForwardValue(StreamMoveForwardValue),
    StreamMovePlaybackSpeedValue(StreamMovePlaybackSpeedValue),
    StreamMoveToTrajectoryValue(StreamMoveToTrajectoryValue),
    StreamStopValue(StreamStopValue),
    StreamMoveBackwardValue(StreamMoveBackwardValue),
}
