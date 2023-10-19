use crate::types::ErrorMessage;
use actix_web::{
    dev::ServiceResponse,
    http::{header, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    Result,
};

pub fn err_handlers<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error)
        .handler(StatusCode::NOT_FOUND, not_found)
}

fn internal_error<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    // split service response into request and response components
    let (req, res) = res.into_parts();

    // set body of response to modified body
    let http_res = serde_json::to_string(&ErrorMessage {
        error: None,
        error_description: None,
        message: "Internal Server Error".to_string(),
    })
    .unwrap_or_else(|_| "{\"message\":\"Internal server error\"}".to_string());
    let res = res.set_body(http_res);

    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

fn not_found<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    // split service response into request and response components
    let (req, res) = res.into_parts();

    // set body of response to modified body
    let http_res = serde_json::to_string(&ErrorMessage {
        error: None,
        error_description: None,
        message: "Not Found".to_string(),
    })
    .unwrap_or_else(|_| "{\"message\":\"Not found\"}".to_string());
    let res = res.set_body(http_res);

    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}
