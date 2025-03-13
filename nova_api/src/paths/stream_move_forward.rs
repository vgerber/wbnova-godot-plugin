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

pub struct StreamMoveForwardStream {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl StreamMoveForwardStream {
    pub fn from(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        StreamMoveForwardStream { socket: socket }
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

pub struct StreamMoveForwardPathParameters {
    pub cell: String,
    pub motion: String,
}
pub struct StreamMoveForwardQueryParameters {
    pub response_coordinate_system: Option<String>,
    pub response_rate: Option<i32>,
    pub playback_speed_in_percent: i32,
    pub start_location_on_trajectory: Option<f64>,
}

pub async fn stream_move_forward(
    host: &str,
    stream_move_forward_path_parameters: &StreamMoveForwardPathParameters,
    stream_move_forward_query_parameters: &StreamMoveForwardQueryParameters,
    additional_headers: Option<Vec<(String, String)>>,
) -> Result<StreamMoveForwardStream, tungstenite::Error> {
    let mut query_parameters: Vec<(&str, String)> = vec![(
        "playback_speed_in_percent",
        stream_move_forward_query_parameters
            .playback_speed_in_percent
            .to_string(),
    )];
    if let Some(ref query_parameter) =
        stream_move_forward_query_parameters.response_coordinate_system
    {
        query_parameters.push(("response_coordinate_system", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = stream_move_forward_query_parameters.response_rate {
        query_parameters.push(("response_rate", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) =
        stream_move_forward_query_parameters.start_location_on_trajectory
    {
        query_parameters.push(("start_location_on_trajectory", query_parameter.to_string()));
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
        "{}/cells/{}/motions/{}/executeforward{}",
        host,
        stream_move_forward_path_parameters.cell,
        stream_move_forward_path_parameters.motion,
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
    Ok(StreamMoveForwardStream::from(socket))
}
