use crate::objects::empty::Empty;
use reqwest;

pub enum PushEStopResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct PushEStopPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn push_e_stop(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &PushEStopPathParameters,
) -> Result<PushEStopResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/estop/push",
            path_parameters.controller, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Empty>().await {
            Ok(empty) => Ok(PushEStopResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(PushEStopResponseType::UndefinedResponse(response)),
    }
}
