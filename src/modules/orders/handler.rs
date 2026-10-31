use std::{collections::BTreeMap, os::linux::raw::stat};

use actix_web::{HttpResponse, web};
use chrono::Utc;
use rust_decimal::Decimal;

use crate::{
  errors::AppError,
  modules::orders::{
    dto::{CreateOrderRequest, CreateOrderResponse, Depth, OrderSide},
    model::{OrderDetail, Orderbook},
  },
  state::AppState,
};

pub async fn create_order(
  state: web::Data<AppState>,
  body: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse, AppError> {
  println!("hi reaching here: {:?}", body);

  let mut orderbook = state.orderbook.lock().unwrap();
  orderbook.create_order(body.into_inner(), String::from("2"));

  Ok(HttpResponse::Ok().json(CreateOrderResponse {
    success: true,
    message: String::from("value"),
  }))
}

pub async fn get_orders(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
  println!("hi reaching get orders");

  // let mut orderbook = Orderbook {
  //   buy: BTreeMap::from([
  //     (
  //       Decimal::from(100),
  //       vec![OrderDetail {
  //         user_id: "user_1".to_string(),
  //         side: OrderSide::Buy,
  //         volume: Decimal::from(5),
  //         created_at: Utc::now(),
  //       }],
  //     ),
  //     (
  //       Decimal::from(110),
  //       vec![OrderDetail {
  //         user_id: "user_3".to_string(),
  //         side: OrderSide::Buy,
  //         volume: Decimal::from(5),
  //         created_at: Utc::now(),
  //       }],
  //     ),
  //   ]),
  //   sell: BTreeMap::from([(
  //     Decimal::from(105),
  //     vec![OrderDetail {
  //       user_id: "user_2".to_string(),
  //       side: OrderSide::Sell,
  //       volume: Decimal::from(10),
  //       created_at: Utc::now(),
  //     }],
  //   )]),
  // };

  let orderbook = state.orderbook.lock().unwrap();
  Ok(HttpResponse::Ok().json(orderbook.to_depth()))
}
