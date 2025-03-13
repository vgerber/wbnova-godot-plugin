use crate::objects::name_list::NameList;
use reqwest;

pub enum ListCellsResponseType {
    Ok(NameList),
    UndefinedResponse(reqwest::Response),
}

pub async fn list_cells(
    client: &reqwest::Client,
    server: &str,
) -> Result<ListCellsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/cells",))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<NameList>().await {
            Ok(name_list) => Ok(ListCellsResponseType::Ok(name_list)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListCellsResponseType::UndefinedResponse(response)),
    }
}
