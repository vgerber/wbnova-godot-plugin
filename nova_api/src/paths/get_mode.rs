use crate::objects::get_mode_response::GetModeResponse;
use reqwest;

pub enum GetModeResponseType {
    Ok(GetModeResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct GetModePathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn get_mode(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetModePathParameters,
) -> Result<GetModeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/mode",
            path_parameters.controller, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<GetModeResponse>().await {
            Ok(get_mode_response) => Ok(GetModeResponseType::Ok(get_mode_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetModeResponseType::UndefinedResponse(response)),
    }
}
