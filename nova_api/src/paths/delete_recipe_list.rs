use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::list_recipe_metadata_response::ListRecipeMetadataResponse;
use reqwest;

pub enum DeleteRecipeListResponseType {
    UnprocessableEntity(HttpValidationError),
    Ok(ListRecipeMetadataResponse),
    NotFound(HttpExceptionResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteRecipeListPathParameters {
    pub cell: String,
}
pub struct DeleteRecipeListQueryParameters {
    pub recipe_ids: Vec<String>,
}

pub async fn delete_recipe_list(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteRecipeListPathParameters,
    query_parameters: &DeleteRecipeListQueryParameters,
) -> Result<DeleteRecipeListResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    query_parameters
        .recipe_ids
        .iter()
        .for_each(|query_parameter_item| {
            request_query_parameters.push(("recipe_ids", query_parameter_item.to_string()))
        });
    let response = match client
        .delete(format!(
            "{server}/cells/{}/store/recipes",
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
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(DeleteRecipeListResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ListRecipeMetadataResponse>().await {
            Ok(list_recipe_metadata_response) => Ok(DeleteRecipeListResponseType::Ok(
                list_recipe_metadata_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => Ok(DeleteRecipeListResponseType::NotFound(
                http_exception_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteRecipeListResponseType::UndefinedResponse(response)),
    }
}
