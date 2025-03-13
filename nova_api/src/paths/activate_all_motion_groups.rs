use crate::objects::motion_group_instance_list::MotionGroupInstanceList;
use reqwest;

pub enum ActivateAllMotionGroupsResponseType {
    Ok(MotionGroupInstanceList),
    UndefinedResponse(reqwest::Response),
}

pub struct ActivateAllMotionGroupsPathParameters {
    pub cell: String,
}
pub struct ActivateAllMotionGroupsQueryParameters {
    pub controller: String,
}

pub async fn activate_all_motion_groups(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ActivateAllMotionGroupsPathParameters,
    query_parameters: &ActivateAllMotionGroupsQueryParameters,
) -> Result<ActivateAllMotionGroupsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> =
        vec![("controller", query_parameters.controller.to_string())];
    let response = match client
        .post(format!(
            "{server}/cells/{}/motion-groups/all",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<MotionGroupInstanceList>().await {
            Ok(motion_group_instance_list) => Ok(ActivateAllMotionGroupsResponseType::Ok(
                motion_group_instance_list,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ActivateAllMotionGroupsResponseType::UndefinedResponse(
            response,
        )),
    }
}
