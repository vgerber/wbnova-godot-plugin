use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::recipe_metadata::RecipeMetadata;
use reqwest;

pub enum GetRecipeMetadataResponseType {
    UnprocessableEntity(HttpValidationError),
    NotFound(HttpExceptionResponse),
    Ok(RecipeMetadata),
    UndefinedResponse(reqwest::Response),
}

pub struct GetRecipeMetadataPathParameters {
    pub cell: String,
    pub recipe: String,
}

pub async fn get_recipe_metadata(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetRecipeMetadataPathParameters,
) -> Result<GetRecipeMetadataResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/recipes/{}/metadata",
            path_parameters.cell, path_parameters.recipe
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
            Ok(http_validation_error) => Ok(GetRecipeMetadataResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => Ok(GetRecipeMetadataResponseType::NotFound(
                http_exception_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<RecipeMetadata>().await {
            Ok(recipe_metadata) => Ok(GetRecipeMetadataResponseType::Ok(recipe_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetRecipeMetadataResponseType::UndefinedResponse(response)),
    }
}
