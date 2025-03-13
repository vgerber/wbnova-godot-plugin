use crate::objects::motion_group_joints::MotionGroupJoints;
use reqwest;

pub enum GetMotionGroupStateResponseType {
    Ok(MotionGroupJoints),
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupStatePathParameters {
    pub cell: String,
    pub id: String,
    pub controller: String,
}

pub async fn get_motion_group_state(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetMotionGroupStatePathParameters,
) -> Result<GetMotionGroupStateResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}",
            path_parameters.cell, path_parameters.id, path_parameters.controller
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<MotionGroupJoints>().await {
            Ok(motion_group_joints) => Ok(GetMotionGroupStateResponseType::Ok(motion_group_joints)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetMotionGroupStateResponseType::UndefinedResponse(response)),
    }
}
