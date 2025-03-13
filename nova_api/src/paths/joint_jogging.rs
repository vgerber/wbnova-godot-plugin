use crate::objects::jogging_response::JoggingResponse;
use crate::objects::joint_jogging_request::JointJoggingRequest;
use reqwest;

pub enum JointJoggingResponseType {
    Ok(JoggingResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct JointJoggingPathParameters {
    pub cell: String,
}

pub async fn joint_jogging(
    client: &reqwest::Client,
    server: &str,
    content: JointJoggingRequest,
    path_parameters: &JointJoggingPathParameters,
) -> Result<JointJoggingResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/move-joint",
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
            Ok(jogging_response) => Ok(JointJoggingResponseType::Ok(jogging_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(JointJoggingResponseType::UndefinedResponse(response)),
    }
}
