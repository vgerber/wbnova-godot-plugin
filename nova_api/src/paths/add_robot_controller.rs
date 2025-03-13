use crate::objects::error::Error;
use crate::objects::robot_controller::RobotController;
use reqwest;

pub enum AddRobotControllerResponseType {
    NotFound(Error),
    Forbidden(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct AddRobotControllerPathParameters {
    pub cell: String,
}
pub struct AddRobotControllerQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn add_robot_controller(
    client: &reqwest::Client,
    server: &str,
    content: RobotController,
    path_parameters: &AddRobotControllerPathParameters,
    query_parameters: &AddRobotControllerQueryParameters,
) -> Result<AddRobotControllerResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .post(format!(
            "{server}/cells/{}/controllers",
            path_parameters.cell
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
            Ok(error) => Ok(AddRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        403 => match response.json::<Error>().await {
            Ok(error) => Ok(AddRobotControllerResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(AddRobotControllerResponseType::UndefinedResponse(response)),
    }
}
