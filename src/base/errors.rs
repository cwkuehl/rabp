use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Error;
use service::ServiceError;

#[derive(Debug, Error)]
pub enum BpError {
    //#[display(fmt = "rabp error")]
    ServiceError(ServiceError),

    //#[display(fmt = "connection error")]
    ConnectionError(r2d2::Error),

    //#[display(fmt = "internal error")]
    InternalError,

    //#[display(fmt = "blocking error")]
    BlockingError(error::BlockingError),
}

impl std::fmt::Display for BpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            BpError::ServiceError(ref e) => write!(f, "rabp error ({})", e),
            BpError::ConnectionError(ref e) => write!(f, "connection error ({})", e),
            BpError::InternalError => write!(f, "internal error"),
            BpError::BlockingError(ref e) => write!(f, "blocking error ({})", e),
        }
    }
}

impl error::ResponseError for BpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            //.body(self.to_string())
            .body(
                serde_json::to_string(&crate::types::ErrorMessage {
                    error: Some("BpError".to_string()),
                    error_description: Some(self.to_string()),
                    message: "Internal Server Error".to_string(),
                })
                .unwrap(),
            )
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            BpError::ServiceError(_) => StatusCode::FAILED_DEPENDENCY,
            BpError::ConnectionError(_) => StatusCode::FAILED_DEPENDENCY,
            BpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            BpError::BlockingError(_) => StatusCode::NOT_IMPLEMENTED,
        }
    }
}

impl std::convert::From<ServiceError> for BpError {
    fn from(item: ServiceError) -> Self {
        BpError::ServiceError(item)
    }
}

impl std::convert::From<r2d2::Error> for BpError {
    fn from(item: r2d2::Error) -> Self {
        BpError::ConnectionError(item)
    }
}

impl std::convert::From<error::BlockingError> for BpError {
    fn from(item: error::BlockingError) -> Self {
        BpError::BlockingError(item)
    }
}
