use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
  pub success: bool,
  pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct Orders {
  pub bids: Vec<[u32; 2]>,
  pub asks: Vec<[u32; 2]>,
  pub last_updated_at: String,
}
