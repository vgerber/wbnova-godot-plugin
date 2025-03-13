use crate::objects::trigger_object::TriggerObject;
use crate::objects::update_trigger_request_body_json::UpdateTriggerRequestBodyJson;
use reqwest;

pub enum UpdateTriggerResponseType {
    Ok(TriggerObject),
    UndefinedResponse(reqwest::Response),
}

pub struct UpdateTriggerPathParameters {
    pub cell: String,
    pub trigger: String,
}

pub async fn update_trigger(
    client: &reqwest::Client,
    server: &str,
    content: UpdateTriggerRequestBodyJson,
    path_parameters: &UpdateTriggerPathParameters,
) -> Result<UpdateTriggerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/operator/triggers/{}",
            path_parameters.cell, path_parameters.trigger
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<TriggerObject>().await {
            Ok(trigger_object) => Ok(UpdateTriggerResponseType::Ok(trigger_object)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UpdateTriggerResponseType::UndefinedResponse(response)),
    }
}
