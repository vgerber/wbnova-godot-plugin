use crate::objects::update_nova_version_request_body_json::UpdateNovaVersionRequestBodyJson;
use reqwest;

pub enum UpdateNovaVersionResponseType {
    UndefinedResponse(reqwest::Response),
}

pub async fn update_nova_version(
    client: &reqwest::Client,
    server: &str,
    content: UpdateNovaVersionRequestBodyJson,
) -> Result<UpdateNovaVersionResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!("{server}/system/update",))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(UpdateNovaVersionResponseType::UndefinedResponse(response)),
    }
}
