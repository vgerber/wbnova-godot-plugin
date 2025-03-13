use crate::objects::flag::Flag;
use reqwest;

pub enum GetEStopResponseType {
    Ok(Flag),
    UndefinedResponse(reqwest::Response),
}

pub struct GetEStopPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn get_e_stop(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetEStopPathParameters,
) -> Result<GetEStopResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/estop",
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
        200 => match response.json::<Flag>().await {
            Ok(flag) => Ok(GetEStopResponseType::Ok(flag)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetEStopResponseType::UndefinedResponse(response)),
    }
}
