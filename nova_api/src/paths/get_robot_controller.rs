use crate::objects::error::Error;
use crate::objects::robot_controller::RobotController;
use reqwest;

pub enum GetRobotControllerResponseType {
    Ok(RobotController),
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct GetRobotControllerPathParameters {
    pub cell: String,
    pub controller: String,
}

pub async fn get_robot_controller(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetRobotControllerPathParameters,
) -> Result<GetRobotControllerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
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
        200 => match response.json::<RobotController>().await {
            Ok(robot_controller) => Ok(GetRobotControllerResponseType::Ok(robot_controller)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetRobotControllerResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetRobotControllerResponseType::UndefinedResponse(response)),
    }
}
