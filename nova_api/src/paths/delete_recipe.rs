use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::recipe_metadata::RecipeMetadata;
use reqwest;

pub enum DeleteRecipeResponseType {
    Ok(RecipeMetadata),
    NotFound(HttpExceptionResponse),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteRecipePathParameters {
    pub recipe: String,
    pub cell: String,
}

pub async fn delete_recipe(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteRecipePathParameters,
) -> Result<DeleteRecipeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/store/recipes/{}",
            path_parameters.recipe, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<RecipeMetadata>().await {
            Ok(recipe_metadata) => Ok(DeleteRecipeResponseType::Ok(recipe_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => {
                Ok(DeleteRecipeResponseType::NotFound(http_exception_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(DeleteRecipeResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteRecipeResponseType::UndefinedResponse(response)),
    }
}
