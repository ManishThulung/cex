use crate::{
  errors::AppError,
  helpers::{jwt::Claims, response::ApiResponse},
  modules::orders::dto::{CancelOrderRequest, CreateOrderRequest, OrderParams},
  state::AppState,
};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use serde_json::json;
use uuid::Uuid;

pub async fn create_order(
  req: HttpRequest,
  state: web::Data<AppState>,
  body: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse, AppError> {
  let claims = req
    .extensions()
    .get::<Claims>()
    .cloned()
    .ok_or(AppError::Unauthorized)?;
  let order_id = Uuid::new_v4();
  let mut orderbook = state.orderbook.lock().unwrap();
  orderbook.create_order(body.into_inner(), claims.sub, order_id);

  Ok(HttpResponse::Ok().json(ApiResponse {
    success: true,
    message: String::from("value"),
    data: Some(json!({ "order_id": order_id })),
  }))
}

pub async fn delete_order(
  state: web::Data<AppState>,
  body: web::Json<CancelOrderRequest>,
  path: web::Path<OrderParams>,
) -> Result<HttpResponse, AppError> {
  let mut orderbook = state.orderbook.lock().unwrap();
  let deleted = orderbook.delete_order(path.order_id, body.0.side);

  if deleted {
    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
      success: true,
      message: String::from("order deleted successfully"),
      data: None,
    }))
  } else {
    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
      success: false,
      message: String::from("cannot delete your order"),
      data: None,
    }))
  }
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
  println!("orderboo: {:?}", *orderbook);
  Ok(HttpResponse::Ok().json(orderbook.to_depth()))
}
