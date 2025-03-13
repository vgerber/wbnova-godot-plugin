use crate::objects::create_trigger_ok_json::CreateTriggerOkJson;
use crate::objects::create_trigger_request_body_json::CreateTriggerRequestBodyJson;
use reqwest;

pub enum CreateTriggerResponseType {
    Ok(CreateTriggerOkJson),
    UndefinedResponse(reqwest::Response),
}

pub struct CreateTriggerPathParameters {
    pub cell: String,
}

pub async fn create_trigger(
    client: &reqwest::Client,
    server: &str,
    content: CreateTriggerRequestBodyJson,
    path_parameters: &CreateTriggerPathParameters,
) -> Result<CreateTriggerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/operator/triggers",
            path_parameters.cell
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
        200 => match response.json::<CreateTriggerOkJson>().await {
            Ok(create_trigger_ok_json) => Ok(CreateTriggerResponseType::Ok(create_trigger_ok_json)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(CreateTriggerResponseType::UndefinedResponse(response)),
    }
}
