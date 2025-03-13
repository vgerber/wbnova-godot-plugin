use crate::objects::list_payloads_response::ListPayloadsResponse;
use reqwest;

pub enum ListPayloadsResponseType {
    Ok(ListPayloadsResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ListPayloadsPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn list_payloads(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListPayloadsPathParameters,
) -> Result<ListPayloadsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/payloads",
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
        200 => match response.json::<ListPayloadsResponse>().await {
            Ok(list_payloads_response) => Ok(ListPayloadsResponseType::Ok(list_payloads_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListPayloadsResponseType::UndefinedResponse(response)),
    }
}
