use crate::objects::direction_jogging_request::DirectionJoggingRequest;
use crate::objects::jogging_response::JoggingResponse;
use reqwest;

pub enum DirectionJoggingResponseType {
    Ok(JoggingResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct DirectionJoggingPathParameters {
    pub cell: String,
}

pub async fn direction_jogging(
    client: &reqwest::Client,
    server: &str,
    content: DirectionJoggingRequest,
    path_parameters: &DirectionJoggingPathParameters,
) -> Result<DirectionJoggingResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/move-tcp",
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
        200 => match response.json::<JoggingResponse>().await {
            Ok(jogging_response) => Ok(DirectionJoggingResponseType::Ok(jogging_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DirectionJoggingResponseType::UndefinedResponse(response)),
    }
}
