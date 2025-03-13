use crate::objects::collision_motion_group_tool::CollisionMotionGroupTool;
use reqwest;

pub enum GetStoredCollisionToolResponseType {
    Ok(CollisionMotionGroupTool),
    UndefinedResponse(reqwest::Response),
}

pub struct GetStoredCollisionToolPathParameters {
    pub cell: String,
    pub tool: String,
}

pub async fn get_stored_collision_tool(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetStoredCollisionToolPathParameters,
) -> Result<GetStoredCollisionToolResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/tools/{}",
            path_parameters.cell, path_parameters.tool
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CollisionMotionGroupTool>().await {
            Ok(collision_motion_group_tool) => Ok(GetStoredCollisionToolResponseType::Ok(
                collision_motion_group_tool,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetStoredCollisionToolResponseType::UndefinedResponse(
            response,
        )),
    }
}
