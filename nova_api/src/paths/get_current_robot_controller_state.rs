use crate::objects::robot_controller_state::RobotControllerState;
use reqwest;

pub enum GetCurrentRobotControllerStateResponseType {
    Ok(RobotControllerState),
    UndefinedResponse(reqwest::Response),
}

pub struct GetCurrentRobotControllerStatePathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn get_current_robot_controller_state(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetCurrentRobotControllerStatePathParameters,
) -> Result<GetCurrentRobotControllerStateResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/state",
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
        200 => match response.json::<RobotControllerState>().await {
            Ok(robot_controller_state) => Ok(GetCurrentRobotControllerStateResponseType::Ok(
                robot_controller_state,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetCurrentRobotControllerStateResponseType::UndefinedResponse(response)),
    }
}
