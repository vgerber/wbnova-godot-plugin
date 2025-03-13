use crate::objects::stream_move_response::StreamMoveResponse;
use http::HeaderName;
use std::net::TcpStream;
use tungstenite::client::IntoClientRequest;
use tungstenite::connect;
use tungstenite::http::Uri;
use tungstenite::protocol::CloseFrame;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Error;
use tungstenite::WebSocket;

pub struct StreamMoveToTrajectoryViaJointPtpStream {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl StreamMoveToTrajectoryViaJointPtpStream {
    pub fn from(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        StreamMoveToTrajectoryViaJointPtpStream { socket: socket }
    }

    pub fn close(&mut self, code: Option<CloseFrame>) -> Result<(), Error> {
        self.socket.close(code)
    }

    pub fn read(&mut self) -> Result<StreamMoveResponse, String> {
        let response = match self.socket.read() {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        let response_text = match response.into_text() {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        let result = match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(response_json_object) => response_json_object,
            Err(err) => return Err(err.to_string()),
        };

        let response_object = match result.get("result") {
            Some(response_object) => response_object,
            None => return Err("No result in message".to_string()),
        };

        match serde_json::from_value::<StreamMoveResponse>(response_object.clone()) {
            Ok(response_object) => Ok(response_object),
            Err(err) => return Err(err.to_string()),
        }
    }
}

pub struct StreamMoveToTrajectoryViaJointPtpPathParameters {
    pub cell: String,
    pub motion: String,
}
pub struct StreamMoveToTrajectoryViaJointPtpQueryParameters {
    pub responses_coordinate_system: Option<String>,
    pub limit_override_joint_velocity_limits_joints: Option<Vec<f64>>,
    pub limit_override_tcp_velocity_limit: Option<f64>,
    pub limit_override_tcp_orientation_velocity_limit: Option<f64>,
    pub limit_override_tcp_acceleration_limit: Option<f64>,
    pub limit_override_tcp_orientation_acceleration_limit: Option<f64>,
    pub location_on_trajectory: f64,
    pub limit_override_joint_acceleration_limits_joints: Option<Vec<f64>>,
}

pub async fn stream_move_to_trajectory_via_joint_ptp(
    host: &str,
    stream_move_to_trajectory_via_joint_ptp_path_parameters: &StreamMoveToTrajectoryViaJointPtpPathParameters,
    stream_move_to_trajectory_via_joint_ptp_query_parameters: &StreamMoveToTrajectoryViaJointPtpQueryParameters,
    additional_headers: Option<Vec<(String, String)>>,
) -> Result<StreamMoveToTrajectoryViaJointPtpStream, tungstenite::Error> {
    let mut query_parameters: Vec<(&str, String)> = vec![(
        "location_on_trajectory",
        stream_move_to_trajectory_via_joint_ptp_query_parameters
            .location_on_trajectory
            .to_string(),
    )];
    if let Some(ref query_parameter) =
        stream_move_to_trajectory_via_joint_ptp_query_parameters.responses_coordinate_system
    {
        query_parameters.push(("responses_coordinate_system", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = stream_move_to_trajectory_via_joint_ptp_query_parameters
        .limit_override_joint_velocity_limits_joints
    {
        query_parameter.iter().for_each(|query_parameter_item| {
            query_parameters.push((
                "limit_override.joint_velocity_limits.joints",
                query_parameter_item.to_string(),
            ))
        });
    }
    if let Some(ref query_parameter) =
        stream_move_to_trajectory_via_joint_ptp_query_parameters.limit_override_tcp_velocity_limit
    {
        query_parameters.push((
            "limit_override.tcp_velocity_limit",
            query_parameter.to_string(),
        ));
    }
    if let Some(ref query_parameter) = stream_move_to_trajectory_via_joint_ptp_query_parameters
        .limit_override_tcp_orientation_velocity_limit
    {
        query_parameters.push((
            "limit_override.tcp_orientation_velocity_limit",
            query_parameter.to_string(),
        ));
    }
    if let Some(ref query_parameter) = stream_move_to_trajectory_via_joint_ptp_query_parameters
        .limit_override_tcp_acceleration_limit
    {
        query_parameters.push((
            "limit_override.tcp_acceleration_limit",
            query_parameter.to_string(),
        ));
    }
    if let Some(ref query_parameter) = stream_move_to_trajectory_via_joint_ptp_query_parameters
        .limit_override_tcp_orientation_acceleration_limit
    {
        query_parameters.push((
            "limit_override.tcp_orientation_acceleration_limit",
            query_parameter.to_string(),
        ));
    }
    if let Some(ref query_parameter) = stream_move_to_trajectory_via_joint_ptp_query_parameters
        .limit_override_joint_acceleration_limits_joints
    {
        query_parameter.iter().for_each(|query_parameter_item| {
            query_parameters.push((
                "limit_override.joint_acceleration_limits.joints",
                query_parameter_item.to_string(),
            ))
        });
    }
    let mut query_string = query_parameters
        .iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<String>>()
        .join("&");
    if query_string.len() > 0 {
        query_string.insert_str(0, "?");
    }
    let url = format!(
        "{}/cells/{}/motions/{}/executetotrajectory{}",
        host,
        stream_move_to_trajectory_via_joint_ptp_path_parameters.cell,
        stream_move_to_trajectory_via_joint_ptp_path_parameters.motion,
        query_string
    );
    let uri: Uri = match url.parse() {
        Ok(uri) => uri,
        Err(err) => return Err(err.into()),
    };

    let mut request = match uri.into_client_request() {
        Ok(request) => request,
        Err(err) => return Err(err),
    };
    let request_headers = request.headers_mut();

    if let Some(additional_headers) = additional_headers {
        additional_headers.iter().for_each(|(key, value)| {
            let key = match HeaderName::try_from(key) {
                Ok(key) => key,
                Err(_) => return (),
            };
            let value = match value.parse() {
                Ok(value) => value,
                Err(_) => return (),
            };
            request_headers.append(key, value);
        });
    }

    let (socket, _) = match connect(request) {
        Ok(connection) => connection,
        Err(err) => return Err(err),
    };
    Ok(StreamMoveToTrajectoryViaJointPtpStream::from(socket))
}
