use crate::objects::pose::Pose;
use crate::objects::tcp_pose_request::TcpPoseRequest;
use reqwest;

pub enum CalculateForwardKinematicResponseType {
    Ok(Pose),
    UndefinedResponse(reqwest::Response),
}

pub struct CalculateForwardKinematicPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn calculate_forward_kinematic(
    client: &reqwest::Client,
    server: &str,
    content: TcpPoseRequest,
    path_parameters: &CalculateForwardKinematicPathParameters,
) -> Result<CalculateForwardKinematicResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/motion-groups/{}/kinematic/calculate-tcp-pose",
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
        200 => match response.json::<Pose>().await {
            Ok(pose) => Ok(CalculateForwardKinematicResponseType::Ok(pose)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CalculateForwardKinematicResponseType::UndefinedResponse(
            response,
        )),
    }
}
