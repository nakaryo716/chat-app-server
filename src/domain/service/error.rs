use axum::response::IntoResponse;
use http::StatusCode;

use crate::domain::repository::error::RepositoryError;

#[derive(Debug, Clone)]
pub enum ServiceError {
    UserAlreadyExist,
    NotFound,
    ToHash,
    Server,
    Validation,
    WrongCredentials,
    TokenCreation,
    TokenVerify,
    MissingCredentials,
    InvalidToken,
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ServiceError::NotFound,
            _ => ServiceError::Server,
        }
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::DbError => Self::Server,
            RepositoryError::NotFound => Self::NotFound,
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
        }
    }
}
