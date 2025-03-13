use crate::objects::motion_group_behavior_getter::MotionGroupBehaviorGetter;
use reqwest;

pub enum GetMotionGroupBehaviorResponseType {
    Ok(MotionGroupBehaviorGetter),
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupBehaviorPathParameters {
    pub id: String,
    pub controller: String,
    pub cell: String,
}

pub async fn get_motion_group_behavior(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetMotionGroupBehaviorPathParameters,
) -> Result<GetMotionGroupBehaviorResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}/behavior",
            path_parameters.id, path_parameters.controller, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<MotionGroupBehaviorGetter>().await {
            Ok(motion_group_behavior_getter) => Ok(GetMotionGroupBehaviorResponseType::Ok(
                motion_group_behavior_getter,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetMotionGroupBehaviorResponseType::UndefinedResponse(
            response,
        )),
    }
}
