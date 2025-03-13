use crate::objects::error::Error;
use reqwest;

pub enum DeleteRobotControllerResponseType {
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteRobotControllerPathParameters {
    pub cell: String,
    pub controller: String,
}
pub struct DeleteRobotControllerQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn delete_robot_controller(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteRobotControllerPathParameters,
    query_parameters: &DeleteRobotControllerQueryParameters,
) -> Result<DeleteRobotControllerResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .delete(format!(
            "{server}/cells/{}/controllers/{}",
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
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteRobotControllerResponseType::UndefinedResponse(
            response,
        )),
    }
}
