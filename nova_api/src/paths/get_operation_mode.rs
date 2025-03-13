use crate::objects::op_mode::OpMode;
use reqwest;

pub enum GetOperationModeResponseType {
    Ok(OpMode),
    UndefinedResponse(reqwest::Response),
}

pub struct GetOperationModePathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn get_operation_mode(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetOperationModePathParameters,
) -> Result<GetOperationModeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
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
        200 => match response.json::<OpMode>().await {
            Ok(op_mode) => Ok(GetOperationModeResponseType::Ok(op_mode)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetOperationModeResponseType::UndefinedResponse(response)),
    }
}
