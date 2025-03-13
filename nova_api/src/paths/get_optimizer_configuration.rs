use crate::objects::optimizer_setup::OptimizerSetup;
use reqwest;

pub enum GetOptimizerConfigurationResponseType {
    Ok(OptimizerSetup),
    UndefinedResponse(reqwest::Response),
}

pub struct GetOptimizerConfigurationPathParameters {
    pub cell: String,
    pub motion_group: String,
}
pub struct GetOptimizerConfigurationQueryParameters {
    pub tcp: Option<String>,
}

pub async fn get_optimizer_configuration(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetOptimizerConfigurationPathParameters,
    query_parameters: &GetOptimizerConfigurationQueryParameters,
) -> Result<GetOptimizerConfigurationResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.tcp {
        request_query_parameters.push(("tcp", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/optimizer-setup",
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
        200 => match response.json::<OptimizerSetup>().await {
            Ok(optimizer_setup) => Ok(GetOptimizerConfigurationResponseType::Ok(optimizer_setup)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetOptimizerConfigurationResponseType::UndefinedResponse(
            response,
        )),
    }
}
