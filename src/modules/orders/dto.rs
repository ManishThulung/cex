use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct OrderParams {
  pub order_id: Uuid,
}

// #[serde(rename_all = "lowercase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
  Buy,
  Sell,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
  pub volume: Decimal,
  pub price: Decimal,
  pub side: OrderSide,
}

#[derive(Debug, Deserialize)]
pub struct CancelOrderRequest {
  // pub volume: Decimal,
  pub side: OrderSide,
}

#[derive(Deserialize, Serialize)]
pub struct Depth {
  pub buy: Vec<[u32; 2]>,
  pub sell: Vec<[u32; 2]>,
  pub last_updated_at: String,
}
