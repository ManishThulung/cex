use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize)]
pub struct OrderParams {
  pub order_id: Uuid,
}

// #[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrderSide {
  Buy,
  Sell,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrderRequest {
  #[validate(custom(function = "validate_positive_decimal"))]
  pub volume: Decimal,

  #[validate(custom(function = "validate_positive_decimal"))]
  pub price: Decimal,

  pub side: OrderSide,
}

fn validate_positive_decimal(value: &Decimal) -> Result<(), validator::ValidationError> {
  if *value <= Decimal::ZERO || *value > Decimal::new(100_000_000, 0) {
    return Err(validator::ValidationError::new("invalid_decimal"));
  }

  Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct Depth {
  pub buy: Vec<[u32; 2]>,
  pub sell: Vec<[u32; 2]>,
  pub last_updated_at: String,
}
