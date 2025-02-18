use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    ValidationError(#[from] garde::Report),
    #[error("トランザクションを実行できませんでした。")]
    TransactionError(#[source] sqlx::Error),
    #[error("データベース処理実行中にエラーが発生しました。")]
    SpecificOperationError(#[source] sqlx::Error),
    #[error("No rows affected: {0}")]
    NoRowsAffectedError(String),
    #[error("{0}")]
    KeyValueStoreError(#[from] redis::RedisError),
    #[error("{0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
    #[error("Open AI API error: {0}")]
    OpenAIError(#[from] async_openai::error::OpenAIError),
    #[error("コサイン類似度計算中にエラーが発生しました。")]
    CosineSimilarityError(String),
    #[error("ログインに失敗しました")]
    UnauthenticatedError,
    #[error("認可情報が誤っています")]
    UnauthorizedError,
    #[error("許可されていない操作です")]
    ForbiddenOperation,
    #[error("{0}")]
    ConversionEntityError(String),
    #[error("コインが不足しています。")]
    InsufficientCoinsError,
    #[error("不正なセッションIDです。")]
    InvalidSessionIdError,
    #[error("コインの追加に失敗しました。")]
    FailedToAddCoinError,
}

impl AppError {
    fn as_str(&self) -> &'static str {
        match self {
            AppError::UnprocessableEntity(_) => "UnprocessableEntity",
            AppError::EntityNotFound(_) => "EntityNotFound",
            AppError::ValidationError(_) => "ValidationError",
            AppError::TransactionError(_) => "TransactionError",
            AppError::SpecificOperationError(_) => "SpecificOperationError",
            AppError::NoRowsAffectedError(_) => "NoRowsAffectedError",
            AppError::KeyValueStoreError(_) => "KeyValueStoreError",
            AppError::BcryptError(_) => "BcryptError",
            AppError::ConvertToUuidError(_) => "ConvertToUuidError",
            AppError::OpenAIError(_) => "OpenAIError",
            AppError::CosineSimilarityError(_) => "CosineSimilarityError",
            AppError::UnauthenticatedError => "UnauthenticatedError",
            AppError::UnauthorizedError => "UnauthorizedError",
            AppError::ForbiddenOperation => "ForbiddenOperation",
            AppError::ConversionEntityError(_) => "ConversionEntityError",
            AppError::InsufficientCoinsError => "InsufficientCoinsError",
            AppError::InvalidSessionIdError => "InvalidSessionIdError",
            AppError::FailedToAddCoinError => "FailedToAddCoinError",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match &self {
            AppError::UnprocessableEntity(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg.to_string())
            }
            AppError::EntityNotFound(msg) => (StatusCode::NOT_FOUND, msg.to_string()),
            AppError::ValidationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::ConvertToUuidError(_) => (
                StatusCode::BAD_REQUEST,
                "Failed to convert to UUID".to_string(),
            ),
            AppError::UnauthenticatedError => (StatusCode::FORBIDDEN, "Access denied".to_string()),
            AppError::ForbiddenOperation => {
                (StatusCode::FORBIDDEN, "Permission denied".to_string())
            }
            AppError::UnauthorizedError => (
                StatusCode::UNAUTHORIZED,
                "Authentication failed".to_string(),
            ),
            AppError::InsufficientCoinsError => {
                (StatusCode::BAD_REQUEST, "Insufficient balance".to_string())
            }
            AppError::InvalidSessionIdError => {
                (StatusCode::BAD_REQUEST, "Invalid session ID".to_string())
            }
            e @ (AppError::TransactionError(_)
            | AppError::SpecificOperationError(_)
            | AppError::NoRowsAffectedError(_)
            | AppError::KeyValueStoreError(_)
            | AppError::BcryptError(_)
            | AppError::OpenAIError(_)
            | AppError::CosineSimilarityError(_)
            | AppError::ConversionEntityError(_)
            | AppError::FailedToAddCoinError) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Unexpected error happened"
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unexpected error happened.".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": {
                "status": status_code.as_u16(),
                "code": self.as_str(),
                "message": error_message,
            }
        }));

        (status_code, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
