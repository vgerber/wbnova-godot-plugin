use crate::objects::mode_change_response::ModeChangeResponse;
use http::HeaderName;
use std::net::TcpStream;
use tungstenite::client::IntoClientRequest;
use tungstenite::connect;
use tungstenite::http::Uri;
use tungstenite::protocol::CloseFrame;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Error;
use tungstenite::WebSocket;

pub struct StreamModeChangeStream {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl StreamModeChangeStream {
    pub fn from(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        StreamModeChangeStream { socket: socket }
    }

    pub fn close(&mut self, code: Option<CloseFrame>) -> Result<(), Error> {
        self.socket.close(code)
    }

    pub fn read(&mut self) -> Result<ModeChangeResponse, String> {
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

        match serde_json::from_value::<ModeChangeResponse>(response_object.clone()) {
            Ok(response_object) => Ok(response_object),
            Err(err) => return Err(err.to_string()),
        }
    }
}

pub struct StreamModeChangePathParameters {
    pub cell: String,
    pub controller: String,
}
pub async fn stream_mode_change(
    host: &str,
    stream_mode_change_path_parameters: &StreamModeChangePathParameters,
    additional_headers: Option<Vec<(String, String)>>,
) -> Result<StreamModeChangeStream, tungstenite::Error> {
    let query_parameters: Vec<(&str, String)> = vec![];
    let mut query_string = query_parameters
        .iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<String>>()
        .join("&");
    if query_string.len() > 0 {
        query_string.insert_str(0, "?");
    }
    let url = format!(
        "{}/cells/{}/controllers/{}/mode-stream{}",
        host,
        stream_mode_change_path_parameters.cell,
        stream_mode_change_path_parameters.controller,
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
    Ok(StreamModeChangeStream::from(socket))
}
