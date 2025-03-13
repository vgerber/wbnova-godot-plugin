use crate::objects::payload::Payload;
use reqwest;

pub enum GetActivePayloadResponseType {
    Ok(Payload),
    UndefinedResponse(reqwest::Response),
}

pub struct GetActivePayloadPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn get_active_payload(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetActivePayloadPathParameters,
) -> Result<GetActivePayloadResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/payloads/current",
            path_parameters.cell, path_parameters.motion_group
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Payload>().await {
            Ok(payload) => Ok(GetActivePayloadResponseType::Ok(payload)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetActivePayloadResponseType::UndefinedResponse(response)),
    }
}
