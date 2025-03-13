use crate::objects::get_all_triggers_ok_json::GetAllTriggersOkJson;
use reqwest;

pub enum GetAllTriggersResponseType {
    Ok(GetAllTriggersOkJson),
    UndefinedResponse(reqwest::Response),
}

pub struct GetAllTriggersPathParameters {
    pub cell: String,
}

pub async fn get_all_triggers(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetAllTriggersPathParameters,
) -> Result<GetAllTriggersResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/operator/triggers",
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
        200 => match response.json::<GetAllTriggersOkJson>().await {
            Ok(get_all_triggers_ok_json) => {
                Ok(GetAllTriggersResponseType::Ok(get_all_triggers_ok_json))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetAllTriggersResponseType::UndefinedResponse(response)),
    }
}
