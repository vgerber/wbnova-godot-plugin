use crate::objects::motion_group_infos::MotionGroupInfos;
use reqwest;

pub enum GetMotionGroupsResponseType {
    Ok(MotionGroupInfos),
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupsPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn get_motion_groups(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetMotionGroupsPathParameters,
) -> Result<GetMotionGroupsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motiongroups",
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
        200 => match response.json::<MotionGroupInfos>().await {
            Ok(motion_group_infos) => Ok(GetMotionGroupsResponseType::Ok(motion_group_infos)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetMotionGroupsResponseType::UndefinedResponse(response)),
    }
}
