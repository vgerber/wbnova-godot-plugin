use crate::objects::error::Error;
use reqwest;

pub enum ClearRobotControllersResponseType {
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct ClearRobotControllersPathParameters {
    pub cell: String,
}
pub struct ClearRobotControllersQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn clear_robot_controllers(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ClearRobotControllersPathParameters,
    query_parameters: &ClearRobotControllersQueryParameters,
) -> Result<ClearRobotControllersResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .delete(format!(
            "{server}/cells/{}/controllers",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ClearRobotControllersResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ClearRobotControllersResponseType::UndefinedResponse(
            response,
        )),
    }
}
