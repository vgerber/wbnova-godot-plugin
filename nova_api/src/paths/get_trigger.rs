use crate::objects::trigger_object::TriggerObject;
use reqwest;

pub enum GetTriggerResponseType {
    Ok(TriggerObject),
    UndefinedResponse(reqwest::Response),
}

pub struct GetTriggerPathParameters {
    pub trigger: String,
    pub cell: String,
}

pub async fn get_trigger(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetTriggerPathParameters,
) -> Result<GetTriggerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/operator/triggers/{}",
            path_parameters.trigger, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<TriggerObject>().await {
            Ok(trigger_object) => Ok(GetTriggerResponseType::Ok(trigger_object)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetTriggerResponseType::UndefinedResponse(response)),
    }
}
