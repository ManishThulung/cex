use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub name: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
  pub name: String,
  pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

use super::model::User;

impl From<User> for UserResponse {
  fn from(user: User) -> Self {
    Self {
      id: user.id,
      name: user.name,
      email: user.email,
      created_at: user.created_at,
      updated_at: user.updated_at,
    }
  }
}
