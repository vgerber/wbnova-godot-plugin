use crate::objects::list_tcps_response::ListTcpsResponse;
use reqwest;

pub enum ListTcpsResponseType {
    Ok(ListTcpsResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ListTcpsPathParameters {
    pub motion_group: String,
    pub cell: String,
}
pub struct ListTcpsQueryParameters {
    pub rotation_type: Option<String>,
}

pub async fn list_tcps(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListTcpsPathParameters,
    query_parameters: &ListTcpsQueryParameters,
) -> Result<ListTcpsResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.rotation_type {
        request_query_parameters.push(("rotation_type", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/tcps",
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
        200 => match response.json::<ListTcpsResponse>().await {
            Ok(list_tcps_response) => Ok(ListTcpsResponseType::Ok(list_tcps_response)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListTcpsResponseType::UndefinedResponse(response)),
    }
}
