use crate::objects::empty::Empty;
use crate::objects::motion_group_joints::MotionGroupJoints;
use reqwest;

pub enum SetMotionGroupStateResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct SetMotionGroupStatePathParameters {
    pub cell: String,
    pub id: String,
    pub controller: String,
}

pub async fn set_motion_group_state(
    client: &reqwest::Client,
    server: &str,
    content: MotionGroupJoints,
    path_parameters: &SetMotionGroupStatePathParameters,
) -> Result<SetMotionGroupStateResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}",
            path_parameters.cell, path_parameters.id, path_parameters.controller
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
        200 => match response.json::<Empty>().await {
            Ok(empty) => Ok(SetMotionGroupStateResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(SetMotionGroupStateResponseType::UndefinedResponse(response)),
    }
}
