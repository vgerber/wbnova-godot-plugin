use crate::objects::name_list::NameList;
use reqwest;

pub enum ListAppsResponseType {
    Ok(NameList),
    UndefinedResponse(reqwest::Response),
}

pub struct ListAppsPathParameters {
    pub cell: String,
}

pub async fn list_apps(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListAppsPathParameters,
) -> Result<ListAppsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/cells/{}/apps", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<NameList>().await {
            Ok(name_list) => Ok(ListAppsResponseType::Ok(name_list)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListAppsResponseType::UndefinedResponse(response)),
    }
}
