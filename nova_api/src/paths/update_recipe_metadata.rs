use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::recipe_metadata::RecipeMetadata;
use crate::objects::update_recipe_metadata_request::UpdateRecipeMetadataRequest;
use reqwest;

pub enum UpdateRecipeMetadataResponseType {
    UnprocessableEntity(HttpValidationError),
    NotFound(HttpExceptionResponse),
    Ok(RecipeMetadata),
    UndefinedResponse(reqwest::Response),
}

pub struct UpdateRecipeMetadataPathParameters {
    pub cell: String,
    pub recipe: String,
}

pub async fn update_recipe_metadata(
    client: &reqwest::Client,
    server: &str,
    content: UpdateRecipeMetadataRequest,
    path_parameters: &UpdateRecipeMetadataPathParameters,
) -> Result<UpdateRecipeMetadataResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/store/recipes/{}/metadata",
            path_parameters.cell, path_parameters.recipe
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(UpdateRecipeMetadataResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => Ok(UpdateRecipeMetadataResponseType::NotFound(
                http_exception_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<RecipeMetadata>().await {
            Ok(recipe_metadata) => Ok(UpdateRecipeMetadataResponseType::Ok(recipe_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UpdateRecipeMetadataResponseType::UndefinedResponse(
            response,
        )),
    }
}
