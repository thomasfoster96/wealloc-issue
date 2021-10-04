#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use async_graphql::{
    Response as GraphQLResponse, 
    ServerError as GraphQLError
};

use worker::*;

#[event(fetch, respond_with_errors)]
async fn fetch(_request: Request, _env: Env) -> Result<Response> {

    let error = GraphQLError::new("Bad Request", None);

    let graphql_response = GraphQLResponse::from_errors(error.into());

    console_log!("Expected: {}", serde_json::to_string(&graphql_response).unwrap());

    let mut response = Response::from_json(&graphql_response)?.with_status(200);

    if graphql_response.is_ok() {
        if let Some(cache_control) = graphql_response.cache_control.value() {
            response
                .headers_mut()
                .set("Cache-Control", &cache_control)?;
        }
    };

    for (name, value) in graphql_response.http_headers.iter() {
        response.headers_mut().append(name.as_str(), &value)?;
    }

    Ok(response)
}
