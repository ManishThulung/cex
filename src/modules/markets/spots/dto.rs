use crate::modules::markets::spots::model::OrderbookStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSpotMarketRequest {
  #[validate(length(min = 1, max = 255))]
  pub name: String,
  #[validate(length(min = 1, max = 255))]
  pub symbol: String,
  #[validate(length(min = 1, max = 255))]
  pub image_url: String,
  #[validate(length(min = 1, max = 255))]
  pub base_symbol: String,
  #[validate(length(min = 1, max = 255))]
  pub quote_symbol: String,
  pub is_visible: bool,
  pub status: OrderbookStatus,
}

// #[derive(Debug, Deserialize)]
// pub struct UpdateUserRequest {
//   pub name: String,
//   pub email: String,
// }

// #[derive(Debug, Serialize)]
// pub struct UserResponse {
//   pub id: Uuid,
//   pub name: String,
//   pub email: String,
//   pub created_at: DateTime<Utc>,
//   pub updated_at: DateTime<Utc>,
// }

// use super::model::User;

// impl From<User> for UserResponse {
//   fn from(user: User) -> Self {
//     Self {
//       id: user.id,
//       name: user.name,
//       email: user.email,
//       created_at: user.created_at,
//       updated_at: user.updated_at,
//     }
//   }
// }
