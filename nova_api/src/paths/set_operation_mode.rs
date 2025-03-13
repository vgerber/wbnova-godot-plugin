use crate::objects::empty::Empty;
use reqwest;

pub enum SetOperationModeResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct SetOperationModePathParameters {
    pub controller: String,
    pub cell: String,
}
pub struct SetOperationModeQueryParameters {
    pub mode: String,
}

pub async fn set_operation_mode(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &SetOperationModePathParameters,
    query_parameters: &SetOperationModeQueryParameters,
) -> Result<SetOperationModeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> =
        vec![("mode", query_parameters.mode.to_string())];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/operationmode",
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
            Ok(empty) => Ok(SetOperationModeResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(SetOperationModeResponseType::UndefinedResponse(response)),
    }
}
