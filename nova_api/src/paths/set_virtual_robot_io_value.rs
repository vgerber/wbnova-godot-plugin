use crate::objects::empty::Empty;
use reqwest;

pub enum SetVirtualRobotIoValueResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct SetVirtualRobotIoValuePathParameters {
    pub io: String,
    pub controller: String,
    pub cell: String,
}
pub struct SetVirtualRobotIoValueQueryParameters {
    pub integer: Option<String>,
    pub bool: Option<bool>,
    pub double: Option<f64>,
}

pub async fn set_virtual_robot_io_value(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &SetVirtualRobotIoValuePathParameters,
    query_parameters: &SetVirtualRobotIoValueQueryParameters,
) -> Result<SetVirtualRobotIoValueResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.integer {
        request_query_parameters.push(("integer", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = query_parameters.bool {
        request_query_parameters.push(("bool", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = query_parameters.double {
        request_query_parameters.push(("double", query_parameter.to_string()));
    }
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/ios/{}",
            path_parameters.io, path_parameters.controller, path_parameters.cell
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
            Ok(empty) => Ok(SetVirtualRobotIoValueResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(SetVirtualRobotIoValueResponseType::UndefinedResponse(
            response,
        )),
    }
}
