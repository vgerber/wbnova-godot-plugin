use crate::objects::empty::Empty;
use reqwest;

pub enum SetMotionGroupBehaviorResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct SetMotionGroupBehaviorPathParameters {
    pub cell: String,
    pub controller: String,
    pub id: String,
}
pub struct SetMotionGroupBehaviorQueryParameters {
    pub behavior: Option<String>,
}

pub async fn set_motion_group_behavior(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &SetMotionGroupBehaviorPathParameters,
    query_parameters: &SetMotionGroupBehaviorQueryParameters,
) -> Result<SetMotionGroupBehaviorResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.behavior {
        request_query_parameters.push(("behavior", query_parameter.to_string()));
    }
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}/behavior",
            path_parameters.cell, path_parameters.controller, path_parameters.id
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Empty>().await {
            Ok(empty) => Ok(SetMotionGroupBehaviorResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(SetMotionGroupBehaviorResponseType::UndefinedResponse(
            response,
        )),
    }
}
