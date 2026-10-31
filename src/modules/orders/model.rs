use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, de};
use uuid::Uuid;

use crate::modules::orders::dto::{CreateOrderRequest, Depth, OrderSide};
use rust_decimal::prelude::ToPrimitive;

#[derive(Debug, Clone, Serialize)]
pub struct Order {
  pub id: Uuid,
  pub side: OrderSide,
  pub volume: Decimal,
  pub price: Decimal,
  pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Orderbook {
  pub buy: BTreeMap<Decimal, Vec<OrderDetail>>,
  pub sell: BTreeMap<Decimal, Vec<OrderDetail>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OrderDetail {
  pub user_id: String,
  pub side: OrderSide,
  pub volume: Decimal,
  pub created_at: DateTime<Utc>,
}

impl Orderbook {
  pub fn new() -> Self {
    Self {
      buy: BTreeMap::new(),
      sell: BTreeMap::new(),
    }
  }

  pub fn create_order(&mut self, order_request: CreateOrderRequest, user_id: String) {
    let order = match order_request.side {
      OrderSide::Buy => &mut self.buy,
      OrderSide::Sell => &mut self.sell,
    };

    let orders = order.entry(order_request.price).or_default();

    // Check whether this user already has an order at this price.
    if let Some(existing_order) = orders.iter_mut().find(|order| order.user_id == user_id) {
      // Update existing order.
      existing_order.volume += order_request.volume;
      existing_order.created_at = Utc::now();
    } else {
      // Create a new order.
      orders.push(OrderDetail {
        user_id,
        side: order_request.side,
        volume: order_request.volume,
        created_at: Utc::now(),
      });
    }
  }

  pub fn to_depth(&self) -> Depth {
    let bids = self
      .buy
      .iter()
      .rev()
      .map(|(price, orders)| {
        let total_volume: Decimal = orders.iter().map(|order| order.volume).sum();

        [
          price.to_u32().unwrap_or(0),
          total_volume.to_u32().unwrap_or(0),
        ]
      })
      .collect();

    let asks = self
      .sell
      .iter()
      .map(|(price, orders)| {
        let total_volume: Decimal = orders.iter().map(|order| order.volume).sum();

        [
          price.to_u32().unwrap_or(0),
          total_volume.to_u32().unwrap_or(0),
        ]
      })
      .collect();

    Depth {
      bids,
      asks,
      last_updated_at: Utc::now().to_rfc3339(),
    }
  }
}
