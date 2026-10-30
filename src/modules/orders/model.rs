use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::modules::orders::dto::OrderSide;

#[derive(Debug, Clone, Serialize)]
pub struct Order {
  pub id: Uuid,
  pub side: OrderSide,
  pub volume: Decimal,
  pub price: Decimal,
  pub created_at: DateTime<Utc>,
}
