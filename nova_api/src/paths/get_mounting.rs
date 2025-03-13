use crate::objects::mounting::Mounting;
use reqwest;

pub enum GetMountingResponseType {
    Ok(Mounting),
    UndefinedResponse(reqwest::Response),
}

pub struct GetMountingPathParameters {
    pub motion_group: String,
    pub cell: String,
}

pub async fn get_mounting(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetMountingPathParameters,
) -> Result<GetMountingResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/mounting",
            path_parameters.motion_group, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Mounting>().await {
            Ok(mounting) => Ok(GetMountingResponseType::Ok(mounting)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetMountingResponseType::UndefinedResponse(response)),
    }
}
