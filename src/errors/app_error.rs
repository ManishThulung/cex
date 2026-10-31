use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  // 400 - Bad Request
  #[error("{0}")]
  Validation(String),

  #[error("{0}")]
  BadRequest(String),

  // 401 - Unauthorized
  #[error("Authentication required")]
  Unauthorized,

  #[error("Invalid credentials")]
  InvalidCredentials,

  #[error("Invalid or expired token")]
  InvalidToken,

  // 403 - Forbidden
  #[error("Access forbidden")]
  Forbidden,

  // 404 - Not Found
  #[error("Resource not found")]
  NotFound,

  // 409 - Conflict
  #[error("Resource conflict")]
  Conflict,

  // 429 - Too Many Requests
  #[error("Too many requests")]
  TooManyRequests,

  // 500 - Internal Server Error
  #[error("Internal server error")]
  InternalServerError,

  // 503 - Service Unavailable
  #[error("Service unavailable")]
  ServiceUnavailable,

  // Database errors
  #[error("Database operation failed")]
  Database(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
  error: String,
}

impl AppError {
  fn status(&self) -> StatusCode {
    match self {
      Self::Validation(_) | Self::BadRequest(_) => StatusCode::BAD_REQUEST,

      Self::Unauthorized | Self::InvalidCredentials | Self::InvalidToken => {
        StatusCode::UNAUTHORIZED
      }

      Self::Forbidden => StatusCode::FORBIDDEN,

      Self::NotFound => StatusCode::NOT_FOUND,

      Self::Conflict => StatusCode::CONFLICT,

      Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,

      Self::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,

      Self::Database(sqlx::Error::Database(error)) if error.code().as_deref() == Some("23505") => {
        StatusCode::CONFLICT
      }

      Self::Database(_) | Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn public_message(&self) -> String {
    match self {
      Self::Validation(message) | Self::BadRequest(message) => message.clone(),

      Self::Unauthorized => "Authentication required".to_string(),

      Self::InvalidCredentials => "Invalid credentials".to_string(),

      Self::InvalidToken => "Invalid or expired token".to_string(),

      Self::Forbidden => "You do not have permission to perform this action".to_string(),

      Self::NotFound => "Resource not found".to_string(),

      Self::Conflict => "Resource already exists".to_string(),

      Self::TooManyRequests => "Too many requests. Please try again later".to_string(),

      Self::ServiceUnavailable => "Service temporarily unavailable".to_string(),

      Self::InternalServerError => "Internal server error".to_string(),

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
