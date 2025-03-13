use crate::objects::stream_move_request_body_json::StreamMoveRequestBodyJson;
use crate::objects::stream_move_response::StreamMoveResponse;
use reqwest;

pub enum StreamMoveResponseType {
    Ok(StreamMoveResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct StreamMovePathParameters {
    pub cell: String,
}

pub async fn stream_move(
    client: &reqwest::Client,
    server: &str,
    content: StreamMoveRequestBodyJson,
    path_parameters: &StreamMovePathParameters,
) -> Result<StreamMoveResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motions/streammove",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<StreamMoveResponse>().await {
            Ok(stream_move_response) => Ok(StreamMoveResponseType::Ok(stream_move_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(StreamMoveResponseType::UndefinedResponse(response)),
    }
}
