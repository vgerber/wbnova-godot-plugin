use crate::objects::motion_group_specification::MotionGroupSpecification;
use reqwest;

pub enum GetMotionGroupSpecificationResponseType {
    Ok(MotionGroupSpecification),
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupSpecificationPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn get_motion_group_specification(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetMotionGroupSpecificationPathParameters,
) -> Result<GetMotionGroupSpecificationResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/specification",
            path_parameters.cell, path_parameters.motion_group
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<MotionGroupSpecification>().await {
            Ok(motion_group_specification) => Ok(GetMotionGroupSpecificationResponseType::Ok(
                motion_group_specification,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetMotionGroupSpecificationResponseType::UndefinedResponse(
            response,
        )),
    }
}
