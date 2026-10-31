use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use sqlx::error::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("{0}")]
  Validation(String),

  #[error("User not found")]
  NotFound,

  #[error("Resource conflict")]
  Conflict,

  #[error("Database operation failed")]
  Database(#[from] sqlx::Error),

  #[error("Invalid credentials")]
  InvalidCredentials,
}

#[derive(Serialize)]
struct ErrorResponse {
  error: String,
}

impl AppError {
  fn status(&self) -> StatusCode {
    match self {
      Self::Validation(_) => StatusCode::BAD_REQUEST,
      Self::NotFound => StatusCode::NOT_FOUND,
      Self::Conflict => StatusCode::CONFLICT,
      Self::InvalidCredentials => StatusCode::BAD_REQUEST,
      Self::Database(sqlx::Error::Database(error)) if error.code().as_deref() == Some("23505") => {
        StatusCode::CONFLICT
      }
      Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn public_message(&self) -> String {
    match self {
      Self::Validation(message) => message.clone(),
      Self::NotFound => "Resource not found".to_string(),
      Self::Conflict => "Resource already exists".to_string(),
      Self::InvalidCredentials => "Invalid credentials".to_string(),
      Self::Database(sqlx::Error::Database(error)) if error.code().as_deref() == Some("23505") => {
        "A record with this unique value already exists".to_string()
      }
      Self::Database(error) => {
        log::error!("Database operation failed: {error}");
        "Internal server error".to_string()
      }
    }
  }
}

impl ResponseError for AppError {
  fn status_code(&self) -> StatusCode {
    self.status()
  }

  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status()).json(ErrorResponse {
      error: self.public_message(),
    })
  }
}
