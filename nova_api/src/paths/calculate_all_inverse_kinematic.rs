use crate::objects::all_joint_positions_request::AllJointPositionsRequest;
use crate::objects::all_joint_positions_response::AllJointPositionsResponse;
use reqwest;

pub enum CalculateAllInverseKinematicResponseType {
    Ok(AllJointPositionsResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct CalculateAllInverseKinematicPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn calculate_all_inverse_kinematic(
    client: &reqwest::Client,
    server: &str,
    content: AllJointPositionsRequest,
    path_parameters: &CalculateAllInverseKinematicPathParameters,
) -> Result<CalculateAllInverseKinematicResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/motion-groups/{}/kinematic/calculate-all-joint-positions",
            path_parameters.cell, path_parameters.motion_group
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
        200 => match response.json::<AllJointPositionsResponse>().await {
            Ok(all_joint_positions_response) => Ok(CalculateAllInverseKinematicResponseType::Ok(
                all_joint_positions_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CalculateAllInverseKinematicResponseType::UndefinedResponse(
            response,
        )),
    }
}
