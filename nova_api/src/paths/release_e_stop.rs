use crate::objects::empty::Empty;
use reqwest;

pub enum ReleaseEStopResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct ReleaseEStopPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn release_e_stop(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ReleaseEStopPathParameters,
) -> Result<ReleaseEStopResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/estop/release",
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
        200 => match response.json::<Empty>().await {
            Ok(empty) => Ok(ReleaseEStopResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ReleaseEStopResponseType::UndefinedResponse(response)),
    }
}
