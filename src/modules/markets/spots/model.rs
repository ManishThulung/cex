use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Spot {
  pub id: Uuid,
  pub name: String,
  pub symbol: String,
  pub image_url: String,
  pub base_symbol: String,
  pub quote_symbol: String,
  pub is_visible: bool,
  pub status: OrderbookStatus,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "orderbook_status", rename_all = "lowercase")]
pub enum OrderbookStatus {
  Active,
  Inactive,
  Suspended,
  Paused,
}
