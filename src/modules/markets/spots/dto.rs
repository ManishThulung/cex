use crate::modules::markets::spots::model::OrderbookStatus;
use serde::Deserialize;
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
