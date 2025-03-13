use crate::objects::joint_position_request::JointPositionRequest;
use crate::objects::joints::Joints;
use reqwest;

pub enum CalculateInverseKinematicResponseType {
    Ok(Joints),
    UndefinedResponse(reqwest::Response),
}

pub struct CalculateInverseKinematicPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn calculate_inverse_kinematic(
    client: &reqwest::Client,
    server: &str,
    content: JointPositionRequest,
    path_parameters: &CalculateInverseKinematicPathParameters,
) -> Result<CalculateInverseKinematicResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/motion-groups/{}/kinematic/calculate-joint-position",
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
        200 => match response.json::<Joints>().await {
            Ok(joints) => Ok(CalculateInverseKinematicResponseType::Ok(joints)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CalculateInverseKinematicResponseType::UndefinedResponse(
            response,
        )),
    }
}
