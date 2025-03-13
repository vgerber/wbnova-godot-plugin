use crate::objects::service_status_list::ServiceStatusList;
use reqwest;

pub enum GetSystemStatusResponseType {
    Ok(ServiceStatusList),
    UndefinedResponse(reqwest::Response),
}

pub async fn get_system_status(
    client: &reqwest::Client,
    server: &str,
) -> Result<GetSystemStatusResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/system/status",))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ServiceStatusList>().await {
            Ok(service_status_list) => Ok(GetSystemStatusResponseType::Ok(service_status_list)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetSystemStatusResponseType::UndefinedResponse(response)),
    }
}
