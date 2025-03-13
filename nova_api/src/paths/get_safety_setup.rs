use crate::objects::safety_setup::SafetySetup;
use reqwest;

pub enum GetSafetySetupResponseType {
    Ok(SafetySetup),
    UndefinedResponse(reqwest::Response),
}

pub struct GetSafetySetupPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn get_safety_setup(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetSafetySetupPathParameters,
) -> Result<GetSafetySetupResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/safety-setup",
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
        200 => match response.json::<SafetySetup>().await {
            Ok(safety_setup) => Ok(GetSafetySetupResponseType::Ok(safety_setup)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetSafetySetupResponseType::UndefinedResponse(response)),
    }
}
