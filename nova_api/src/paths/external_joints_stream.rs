use crate::objects::external_joint_stream_datapoint::ExternalJointStreamDatapoint;
use crate::objects::motion_group_joints::MotionGroupJoints;
use reqwest;

pub enum ExternalJointsStreamResponseType {
    Ok(MotionGroupJoints),
    UndefinedResponse(reqwest::Response),
}

pub struct ExternalJointsStreamPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn external_joints_stream(
    client: &reqwest::Client,
    server: &str,
    content: ExternalJointStreamDatapoint,
    path_parameters: &ExternalJointsStreamPathParameters,
) -> Result<ExternalJointsStreamResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/externalJointsStream",
            path_parameters.controller, path_parameters.cell
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
        200 => match response.json::<MotionGroupJoints>().await {
            Ok(motion_group_joints) => {
                Ok(ExternalJointsStreamResponseType::Ok(motion_group_joints))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ExternalJointsStreamResponseType::UndefinedResponse(
            response,
        )),
    }
}
