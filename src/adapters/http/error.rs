use axum::{
    extract::{path::ErrorKind, rejection::PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::domain;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    error: String,
}

impl From<&AppError> for ErrorResponse {
    fn from(value: &AppError) -> Self {
        match *value {
            AppError::EntityNotFound(ref error) => Self {
                error: error.clone(),
            },
            AppError::Unexpected(ref error) => Self {
                error: error.clone(),
            },
            AppError::PathParseError(ref rejection) => Self {
                error: rejection.to_string(),
            },
            AppError::Unauthorized(ref error) => Self {
                error: error.clone(),
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    PathParseError(PathRejection),
    #[error("{0}")]
    Unexpected(String),
}

impl From<domain::user::Error> for AppError {
    fn from(value: domain::user::Error) -> Self {
        match value {
            domain::user::Error::UserNotFound(_) => Self::EntityNotFound(value.to_string()),
            domain::user::Error::Unexpected => Self::Unexpected(value.to_string()),
        }
    }
}

impl From<domain::auth::Error> for AppError {
    fn from(value: domain::auth::Error) -> Self {
        match value {
            domain::auth::Error::AuthUserNotFound(_) | domain::auth::Error::UserNotFound(_) => {
                Self::EntityNotFound(value.to_string())
            }
            domain::auth::Error::Unexpected => Self::Unexpected(value.to_string()),
            domain::auth::Error::InvalidAuth => Self::Unauthorized(value.to_string()),
        }
    }
}

impl From<domain::recipe::Error> for AppError {
    fn from(value: domain::recipe::Error) -> Self {
        match value {
            domain::recipe::Error::RecipeNotFound(_) => Self::EntityNotFound(value.to_string()),
            domain::recipe::Error::Unexpected => Self::Unexpected(value.to_string()),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse::from(&self));
        let status = match self {
            Self::EntityNotFound(_) => StatusCode::NOT_FOUND,
            Self::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PathParseError(rejection) => match rejection {
                PathRejection::FailedToDeserializePathParams(inner) => {
                    let kind = inner.into_kind();
                    match &kind {
                        ErrorKind::UnsupportedType { .. } => StatusCode::INTERNAL_SERVER_ERROR,
                        _ => StatusCode::BAD_REQUEST,
                    }
                }
                PathRejection::MissingPathParams(_) => StatusCode::INTERNAL_SERVER_ERROR,

                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        };
        (status, body).into_response()
    }
}

#[cfg(test)]
mod test {

    use axum::body;
    use serde_json::{json, Value};

    use super::*;

    #[tokio::test]
    async fn test_into_response_for_app_error() {
        let app_err: AppError = domain::user::Error::UserNotFound(4).into();
        let response = app_err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: Value = serde_json::from_slice(&body.to_vec()).unwrap();
        assert_eq!(json, json!({"error": "user with id `4` not found"}));
    }
}
