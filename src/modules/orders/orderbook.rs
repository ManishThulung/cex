//! Storage only. The book keeps resting orders in price-time order and answers
//! questions about them. It knows nothing about trades, self-trade rules or
//! sequence numbers: all matching decisions live in `engine.rs`.

use crate::modules::orders::dto::{Depth, OrderSide};
use crate::modules::orders::types::RestingOrder;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::{BTreeMap, HashMap, VecDeque};
use uuid::Uuid;

/// Where a resting order lives, so lookups and cancels skip the full scan.
#[derive(Debug, Clone, Copy)]
struct Locator {
  side: OrderSide,
  price: Decimal,
}

#[derive(Debug, Default)]
pub struct Orderbook {
  /// price -> FIFO queue. Best bid = highest key.
  bids: BTreeMap<Decimal, VecDeque<RestingOrder>>,
  /// price -> FIFO queue. Best ask = lowest key.
  asks: BTreeMap<Decimal, VecDeque<RestingOrder>>,
  /// order_id -> (side, price).
  index: HashMap<Uuid, Locator>,
}

impl Orderbook {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn best_bid(&self) -> Option<Decimal> {
    self.best_price(OrderSide::Buy)
  }

  pub fn best_ask(&self) -> Option<Decimal> {
    self.best_price(OrderSide::Sell)
  }

  pub fn contains(&self, order_id: Uuid) -> bool {
    self.index.contains_key(&order_id)
  }

  /// Look up a resting order by id.
  pub fn get(&self, order_id: Uuid) -> Option<&RestingOrder> {
    let loc = self.index.get(&order_id)?;
    let book = match loc.side {
      OrderSide::Buy => &self.bids,
      OrderSide::Sell => &self.asks,
    };
    book
      .get(&loc.price)?
      .iter()
      .find(|o| o.order_id == order_id)
  }

  /// The first order in the queue at the best price on `side`
  /// (highest bid for Buy, lowest ask for Sell).
  pub fn peek_best(&self, side: OrderSide) -> Option<&RestingOrder> {
    let price = self.best_price(side)?;
    let book = match side {
      OrderSide::Buy => &self.bids,
      OrderSide::Sell => &self.asks,
    };
    book.get(&price)?.front()
  }

  /// Reduce the front order at the best price on `side` by `qty`.
  /// If it reaches zero it is removed, and an emptied price level is dropped.
  pub fn fill_best(&mut self, side: OrderSide, qty: Decimal) {
    let Some(price) = self.best_price(side) else {
      return;
    };

    // Direct field access keeps the borrows of `book` and `self.index` disjoint.
    let book = match side {
      OrderSide::Buy => &mut self.bids,
      OrderSide::Sell => &mut self.asks,
    };
    let Some(queue) = book.get_mut(&price) else {
      return;
    };
    let Some(front) = queue.front_mut() else {
      return;
    };

    front.remaining -= qty;
    if front.remaining <= Decimal::ZERO {
      let done = queue.pop_front().expect("front exists");
      self.index.remove(&done.order_id);
    }

    if queue.is_empty() {
      book.remove(&price);
    }
  }

  /// Add an order at the back of its price level.
  pub fn rest(&mut self, order: RestingOrder) {
    self.index.insert(
      order.order_id,
      Locator {
        side: order.side,
        price: order.price,
      },
    );

    let book = match order.side {
      OrderSide::Buy => &mut self.bids,
      OrderSide::Sell => &mut self.asks,
    };
    book.entry(order.price).or_default().push_back(order);
  }

  /// Remove an order by id, wherever it sits in its queue.
  pub fn remove(&mut self, order_id: Uuid) -> Option<RestingOrder> {
    let loc = *self.index.get(&order_id)?;

    let book = match loc.side {
      OrderSide::Buy => &mut self.bids,
      OrderSide::Sell => &mut self.asks,
    };
    let queue = book.get_mut(&loc.price)?;
    let pos = queue.iter().position(|o| o.order_id == order_id)?;
    let order = queue.remove(pos)?;

    if queue.is_empty() {
      book.remove(&loc.price);
    }
    self.index.remove(&order_id);

    Some(order)
  }

  pub fn to_depth(&self) -> Depth {
    Depth {
      buy: Self::side_depth(self.bids.iter().rev()),
      sell: Self::side_depth(self.asks.iter()),
      last_updated_at: Utc::now().to_rfc3339(),
    }
  }

  fn best_price(&self, side: OrderSide) -> Option<Decimal> {
    match side {
      OrderSide::Buy => self.bids.keys().next_back().copied(),
      OrderSide::Sell => self.asks.keys().next().copied(),
    }
  }

  fn side_depth<'a>(
    levels: impl Iterator<Item = (&'a Decimal, &'a VecDeque<RestingOrder>)>,
  ) -> Vec<[u32; 2]> {
    levels
      .map(|(price, orders)| {
        let total: Decimal = orders.iter().map(|o| o.remaining).sum();
        [price.to_u32().unwrap_or(0), total.to_u32().unwrap_or(0)]
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn d(n: i64) -> Decimal {
    Decimal::from(n)
  }

  fn order(side: OrderSide, price: i64, vol: i64, user: &str) -> RestingOrder {
    RestingOrder {
      order_id: Uuid::new_v4(),
      user_id: user.to_string(),
      side,
      price: d(price),
      original_volume: d(vol),
      remaining: d(vol),
      seq: 0,
      created_at: Utc::now(),
    }
  }

  #[test]
  fn best_prices_follow_side_rules() {
    let mut book = Orderbook::new();
    book.rest(order(OrderSide::Buy, 98, 1, "a"));
    book.rest(order(OrderSide::Buy, 99, 1, "a"));
    book.rest(order(OrderSide::Sell, 101, 1, "b"));
    book.rest(order(OrderSide::Sell, 102, 1, "b"));

    assert_eq!(book.best_bid(), Some(d(99)));
    assert_eq!(book.best_ask(), Some(d(101)));
  }

  #[test]
  fn fifo_within_a_level() {
    let mut book = Orderbook::new();
    book.rest(order(OrderSide::Sell, 100, 5, "a"));
    book.rest(order(OrderSide::Sell, 100, 5, "b"));

    assert_eq!(book.peek_best(OrderSide::Sell).unwrap().user_id, "a");
    book.fill_best(OrderSide::Sell, d(5));
    assert_eq!(book.peek_best(OrderSide::Sell).unwrap().user_id, "b");
  }

  #[test]
  fn partial_fill_keeps_the_same_front_order() {
    let mut book = Orderbook::new();
    let o = order(OrderSide::Sell, 100, 5, "a");
    let id = o.order_id;
    book.rest(o);

    book.fill_best(OrderSide::Sell, d(2));

    let front = book.peek_best(OrderSide::Sell).unwrap();
    assert_eq!(front.order_id, id);
    assert_eq!(front.remaining, d(3));
    assert_eq!(front.original_volume, d(5));
  }

  #[test]
  fn full_fill_cleans_level_and_index() {
    let mut book = Orderbook::new();
    let o = order(OrderSide::Sell, 100, 5, "a");
    let id = o.order_id;
    book.rest(o);

    book.fill_best(OrderSide::Sell, d(5));

    assert_eq!(book.best_ask(), None);
    assert!(!book.contains(id));
  }

  #[test]
  fn remove_takes_from_the_middle_and_cleans_up() {
    let mut book = Orderbook::new();
    let a = order(OrderSide::Buy, 100, 1, "a");
    let b = order(OrderSide::Buy, 100, 1, "b");
    let c = order(OrderSide::Buy, 100, 1, "c");
    let (ida, idb, idc) = (a.order_id, b.order_id, c.order_id);
    book.rest(a);
    book.rest(b);
    book.rest(c);

    assert_eq!(book.remove(idb).unwrap().user_id, "b");
    assert!(book.remove(idb).is_none());
    assert_eq!(book.peek_best(OrderSide::Buy).unwrap().order_id, ida);

    book.remove(ida);
    book.remove(idc);
    assert_eq!(book.best_bid(), None);
  }

  #[test]
  fn depth_sums_remaining_volume_per_level() {
    let mut book = Orderbook::new();
    book.rest(order(OrderSide::Sell, 100, 3, "a"));
    book.rest(order(OrderSide::Sell, 100, 4, "b"));
    book.fill_best(OrderSide::Sell, d(1));

    let depth = book.to_depth();
    assert_eq!(depth.sell, vec![[100, 6]]);
  }
}
