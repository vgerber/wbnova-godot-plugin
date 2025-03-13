use crate::objects::motion_group_instance_list::MotionGroupInstanceList;
use reqwest;

pub enum ListMotionGroupsResponseType {
    Ok(MotionGroupInstanceList),
    UndefinedResponse(reqwest::Response),
}

pub struct ListMotionGroupsPathParameters {
    pub cell: String,
}

pub async fn list_motion_groups(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListMotionGroupsPathParameters,
) -> Result<ListMotionGroupsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups",
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
            Ok(motion_group_instance_list) => {
                Ok(ListMotionGroupsResponseType::Ok(motion_group_instance_list))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListMotionGroupsResponseType::UndefinedResponse(response)),
    }
}
