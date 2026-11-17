use crate::modules::orders::dto::OrderSide;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use std::fmt;
use uuid::Uuid;

/// The other side of the book.
pub fn opposite(side: OrderSide) -> OrderSide {
  match side {
    OrderSide::Buy => OrderSide::Sell,
    OrderSide::Sell => OrderSide::Buy,
  }
}

/// An order that is in the book (or is about to be matched against it).
/// One entry per order: it keeps its own id, queue position and fill state.
#[derive(Debug, Clone)]
pub struct RestingOrder {
  pub order_id: Uuid,
  pub user_id: String,
  pub side: OrderSide,
  pub price: Decimal,
  pub original_volume: Decimal,
  pub remaining: Decimal,
  /// Engine sequence number: the real tie-breaker for time priority.
  pub seq: u64,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Trade {
  pub trade_id: Uuid,
  pub seq: u64,
  /// Always the resting (maker) order's price.
  pub price: Decimal,
  pub volume: Decimal,
  pub taker_side: OrderSide,
  pub maker_order_id: Uuid,
  pub taker_order_id: Uuid,
  // Not exposed in API responses: don't leak counterparties to each other.
  #[serde(skip_serializing)]
  pub maker_user_id: String,
  #[serde(skip_serializing)]
  pub taker_user_id: String,
  pub executed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
  /// No fills, now resting in the book.
  Resting,
  /// Some fills, the remainder is resting in the book.
  PartiallyFilled,
  /// Fully filled, nothing rests.
  Filled,
  /// Matching stopped because the next resting order is the same user's.
  /// Any unfilled remainder was cancelled and does NOT rest.
  SelfTradePrevented,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaceResult {
  pub order_id: Uuid,
  pub status: OrderStatus,
  pub filled: Decimal,
  pub remaining: Decimal,
  pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderError {
  InvalidPrice,
  InvalidVolume,
  DuplicateOrderId,
}

impl fmt::Display for OrderError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      OrderError::InvalidPrice => write!(f, "price must be greater than zero"),
      OrderError::InvalidVolume => write!(f, "volume must be greater than zero"),
      OrderError::DuplicateOrderId => write!(f, "order id already exists"),
    }
  }
}

impl std::error::Error for OrderError {}
