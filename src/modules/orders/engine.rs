use crate::errors::AppError;
use crate::modules::orders::dto::{Depth, OrderSide};
use crate::modules::orders::orderbook::Orderbook;
use crate::modules::orders::types::{
  OrderError, OrderStatus, PlaceResult, RestingOrder, Trade, opposite,
};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct MatchingEngine {
  book: Orderbook,
  /// Monotonic counter shared by orders and trades.
  seq: u64,
}

impl MatchingEngine {
  pub fn new() -> Self {
    Self::default()
  }

  /// Place a limit order: match whatever crosses, rest the remainder.
  ///
  /// Call this while holding the engine's lock for the whole call, so each
  /// order is matched to completion before the next one starts.
  pub fn place_order(
    &mut self,
    side: OrderSide,
    price: Decimal,
    volume: Decimal,
    user_id: String,
    order_id: Uuid,
  ) -> Result<PlaceResult, OrderError> {
    if self.book.contains(order_id) {
      return Err(OrderError::DuplicateOrderId);
    }

    self.seq += 1;
    let mut order = RestingOrder {
      order_id,
      user_id,
      side,
      price,
      original_volume: volume,
      remaining: volume,
      seq: self.seq,
      created_at: Utc::now(),
    };

    let maker_side = opposite(side);
    let mut trades = Vec::new();
    let mut self_trade_hit = false;

    while !order.remaining.is_zero() {
      let Some(offer) = self.book.peek_best(maker_side) else {
        break;
      };

      let crosses = match side {
        OrderSide::Buy => offer.price <= order.price,
        OrderSide::Sell => offer.price >= order.price,
      };
      if !crosses {
        break;
      }

      if offer.user_id == order.user_id {
        self_trade_hit = true;
        break;
      }

      // Copy what we need out of `maker` before touching the book again.
      let qty = order.remaining.min(offer.remaining);
      let trade_price = offer.price;
      let maker_order_id = offer.order_id;
      let maker_user_id = offer.user_id.clone();

      self.book.fill_best(maker_side, qty);
      order.remaining -= qty;

      self.seq += 1;
      trades.push(Trade {
        trade_id: Uuid::new_v4(),
        seq: self.seq,
        price: trade_price, // maker's price
        volume: qty,
        taker_side: side,
        maker_order_id,
        taker_order_id: order.order_id,
        maker_user_id,
        taker_user_id: order.user_id.clone(),
        executed_at: Utc::now(),
      });
    }

    let filled = order.original_volume - order.remaining;
    let remaining = order.remaining;

    let status = if remaining.is_zero() {
      OrderStatus::Filled
    } else if self_trade_hit {
      // Cancel-newest policy: the leftover is dropped, not rested.
      OrderStatus::SelfTradePrevented
    } else {
      let status = if filled.is_zero() {
        OrderStatus::Resting
      } else {
        OrderStatus::PartiallyFilled
      };
      self.book.rest(order);
      status
    };

    Ok(PlaceResult {
      order_id,
      status,
      filled,
      remaining,
      trades,
    })
  }

  pub fn cancel_order(&mut self, order_id: Uuid, user_id: &str) -> Result<RestingOrder, AppError> {
    let owned = match self.book.get(order_id) {
      Some(order) => order.user_id == user_id,
      None => return Err(AppError::NotFound),
    };
    if !owned {
      return Err(AppError::Forbidden);
    }
    self.book.remove(order_id).ok_or(AppError::NotFound)
  }

  pub fn depth(&self) -> Depth {
    self.book.to_depth()
  }

  pub fn best_bid(&self) -> Option<Decimal> {
    self.book.best_bid()
  }

  pub fn best_ask(&self) -> Option<Decimal> {
    self.book.best_ask()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn d(n: i64) -> Decimal {
    Decimal::from(n)
  }

  fn place(
    engine: &mut MatchingEngine,
    side: OrderSide,
    price: i64,
    vol: i64,
    user: &str,
  ) -> PlaceResult {
    engine
      .place_order(side, d(price), d(vol), user.to_string(), Uuid::new_v4())
      .unwrap()
  }

  #[test]
  fn rests_when_nothing_crosses() {
    let mut e = MatchingEngine::new();
    place(&mut e, OrderSide::Sell, 101, 5, "a");
    let r = place(&mut e, OrderSide::Buy, 100, 5, "b");

    assert_eq!(r.status, OrderStatus::Resting);
    assert!(r.trades.is_empty());
    assert_eq!(e.best_bid(), Some(d(100)));
    assert_eq!(e.best_ask(), Some(d(101)));
  }

  #[test]
  fn trades_at_maker_price() {
    let mut e = MatchingEngine::new();
    place(&mut e, OrderSide::Sell, 100, 5, "a");
    let r = place(&mut e, OrderSide::Buy, 105, 5, "b");

    assert_eq!(r.status, OrderStatus::Filled);
    assert_eq!(r.trades.len(), 1);
    assert_eq!(r.trades[0].price, d(100));
    assert_eq!(e.best_ask(), None);
    assert_eq!(e.best_bid(), None);
  }

  #[test]
  fn partial_fill_keeps_time_priority() {
    let mut e = MatchingEngine::new();
    place(&mut e, OrderSide::Sell, 100, 5, "a");
    place(&mut e, OrderSide::Sell, 100, 5, "b");

    let r1 = place(&mut e, OrderSide::Buy, 100, 3, "c");
    assert_eq!(r1.trades.len(), 1);
    assert_eq!(r1.trades[0].maker_user_id, "a");

    // "a" still has 2 left and must still be first in the queue.
    let r2 = place(&mut e, OrderSide::Buy, 100, 4, "d");
    assert_eq!(r2.trades.len(), 2);
    assert_eq!(r2.trades[0].maker_user_id, "a");
    assert_eq!(r2.trades[0].volume, d(2));
    assert_eq!(r2.trades[1].maker_user_id, "b");
    assert_eq!(r2.trades[1].volume, d(2));
  }

  #[test]
  fn sweeps_multiple_levels_and_rests_remainder() {
    let mut e = MatchingEngine::new();
    place(&mut e, OrderSide::Sell, 100, 2, "a");
    place(&mut e, OrderSide::Sell, 101, 2, "b");

    let r = place(&mut e, OrderSide::Buy, 101, 5, "c");
    assert_eq!(r.trades.len(), 2);
    assert_eq!(r.trades[0].price, d(100));
    assert_eq!(r.trades[1].price, d(101));
    assert_eq!(r.status, OrderStatus::PartiallyFilled);
    assert_eq!(r.remaining, d(1));
    assert_eq!(e.best_bid(), Some(d(101)));
    assert_eq!(e.best_ask(), None);
  }

  #[test]
  fn sell_side_taker_matches_highest_bids_first() {
    let mut e = MatchingEngine::new();
    place(&mut e, OrderSide::Buy, 99, 2, "a");
    place(&mut e, OrderSide::Buy, 100, 2, "b");

    let r = place(&mut e, OrderSide::Sell, 99, 3, "c");
    assert_eq!(r.trades.len(), 2);
    assert_eq!(r.trades[0].price, d(100));
    assert_eq!(r.trades[1].price, d(99));
    assert_eq!(r.status, OrderStatus::Filled);
    assert_eq!(e.best_bid(), Some(d(99)));
  }

  // #[test]
  // fn cancel_checks_ownership_and_is_idempotent() {
  //   let mut e = MatchingEngine::new();
  //   let r = place(&mut e, OrderSide::Sell, 100, 5, "a");

  //   assert_eq!(
  //     e.cancel_order(r.order_id, "b").unwrap_err(),
  //     AppError::Forbidden
  //   );
  //   assert!(e.cancel_order(r.order_id, "a").is_ok());
  //   assert_eq!(
  //     e.cancel_order(r.order_id, "a").unwrap_err(),
  //     AppError::NotFound
  //   );
  //   assert_eq!(e.best_ask(), None);
  // }

  #[test]
  fn cancelling_a_partially_filled_order_removes_only_the_remainder() {
    let mut e = MatchingEngine::new();
    let r = place(&mut e, OrderSide::Sell, 100, 5, "a");
    place(&mut e, OrderSide::Buy, 100, 2, "b");

    let cancelled = e.cancel_order(r.order_id, "a").unwrap();
    assert_eq!(cancelled.remaining, d(3));
    assert_eq!(e.best_ask(), None);
  }

  #[test]
  fn self_trade_is_prevented() {
    let mut e = MatchingEngine::new();
    place(&mut e, OrderSide::Sell, 100, 5, "a");
    let r = place(&mut e, OrderSide::Buy, 100, 5, "a");

    assert_eq!(r.status, OrderStatus::SelfTradePrevented);
    assert!(r.trades.is_empty());
    assert_eq!(e.best_ask(), Some(d(100)));
    assert_eq!(e.best_bid(), None);
  }

  #[test]
  fn rejects_invalid_input_and_duplicate_ids() {
    let mut e = MatchingEngine::new();
    let id = Uuid::new_v4();

    assert_eq!(
      e.place_order(OrderSide::Buy, d(0), d(1), "a".into(), id)
        .unwrap_err(),
      OrderError::InvalidPrice
    );
    assert_eq!(
      e.place_order(OrderSide::Buy, d(1), d(0), "a".into(), id)
        .unwrap_err(),
      OrderError::InvalidVolume
    );

    e.place_order(OrderSide::Buy, d(1), d(1), "a".into(), id)
      .unwrap();
    assert_eq!(
      e.place_order(OrderSide::Buy, d(1), d(1), "a".into(), id)
        .unwrap_err(),
      OrderError::DuplicateOrderId
    );
  }

  #[test]
  fn book_never_ends_up_crossed() {
    let mut e = MatchingEngine::new();
    let orders = [
      (OrderSide::Buy, 100, 3),
      (OrderSide::Sell, 102, 4),
      (OrderSide::Sell, 99, 2),
      (OrderSide::Buy, 103, 6),
      (OrderSide::Sell, 98, 5),
      (OrderSide::Buy, 97, 1),
    ];
    for (i, (side, price, vol)) in orders.into_iter().enumerate() {
      place(&mut e, side, price, vol, &format!("u{i}"));
      if let (Some(bid), Some(ask)) = (e.best_bid(), e.best_ask()) {
        assert!(bid < ask, "book crossed: bid {bid} >= ask {ask}");
      }
    }
  }
}
