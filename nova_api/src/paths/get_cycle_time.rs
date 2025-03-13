use crate::objects::cycle_time::CycleTime;
use reqwest;

pub enum GetCycleTimeResponseType {
    Ok(CycleTime),
    UndefinedResponse(reqwest::Response),
}

pub struct GetCycleTimePathParameters {
    pub cell: String,
    pub controller: String,
}

pub async fn get_cycle_time(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetCycleTimePathParameters,
) -> Result<GetCycleTimeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/cycle-time",
            path_parameters.cell, path_parameters.controller
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CycleTime>().await {
            Ok(cycle_time) => Ok(GetCycleTimeResponseType::Ok(cycle_time)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetCycleTimeResponseType::UndefinedResponse(response)),
    }
}
