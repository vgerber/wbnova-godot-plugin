use crate::objects::cell::Cell;
use crate::objects::error::Error;
use reqwest;

pub enum GetCellResponseType {
    Ok(Cell),
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct GetCellPathParameters {
    pub cell: String,
}

pub async fn get_cell(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetCellPathParameters,
) -> Result<GetCellResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/cells/{}", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Cell>().await {
            Ok(cell) => Ok(GetCellResponseType::Ok(cell)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCellResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetCellResponseType::UndefinedResponse(response)),
    }
}
