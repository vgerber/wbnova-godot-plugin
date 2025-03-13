use crate::objects::error::Error;
use crate::objects::robot_controller::RobotController;
use reqwest;

pub enum UpdateRobotControllerResponseType {
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct UpdateRobotControllerPathParameters {
    pub cell: String,
    pub controller: String,
}
pub struct UpdateRobotControllerQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn update_robot_controller(
    client: &reqwest::Client,
    server: &str,
    content: RobotController,
    path_parameters: &UpdateRobotControllerPathParameters,
    query_parameters: &UpdateRobotControllerQueryParameters,
) -> Result<UpdateRobotControllerResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}",
            path_parameters.cell, path_parameters.controller
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UpdateRobotControllerResponseType::UndefinedResponse(
            response,
        )),
    }
}
