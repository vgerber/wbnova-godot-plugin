use reqwest;

pub enum DeleteTriggerResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteTriggerPathParameters {
    pub trigger: String,
    pub cell: String,
}

pub async fn delete_trigger(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteTriggerPathParameters,
) -> Result<DeleteTriggerResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
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
        _ => Ok(DeleteTriggerResponseType::UndefinedResponse(response)),
    }
}
