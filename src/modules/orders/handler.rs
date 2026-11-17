use crate::{
  errors::AppError,
  helpers::{jwt::Claims, response::ApiResponse},
  modules::orders::dto::{CreateOrderRequest, OrderParams},
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
  let payload = body.into_inner();

  let result = {
    let mut orderbook = state.orderbook.lock().unwrap();
    orderbook.place_order(
      payload.side,
      payload.price,
      payload.volume,
      claims.sub,
      order_id,
    )
  };

  match result {
    Ok(place_result) => Ok(HttpResponse::Created().json(ApiResponse {
      success: true,
      message: String::from("order created successfully"),
      data: Some(json!({
        "order_id": order_id,
        "trades": place_result.trades,
        // "remaining_volume": place_result.remaining_volume,
      })),
    })),
    // Err(error) => Err(AppError::from(error)),
    Err(error) => Ok(HttpResponse::BadRequest().json(ApiResponse::<()> {
      success: false,
      message: format!("Failed to create order: {error:?}"),
      data: None,
    })),
  }

  // Ok(HttpResponse::Ok().json(ApiResponse {
  //   success: true,
  //   message: String::from("value"),
  //   data: Some(json!({ "order_id": order_id })),
  // }))
}

pub async fn delete_order(
  req: HttpRequest,
  state: web::Data<AppState>,
  path: web::Path<OrderParams>,
) -> Result<HttpResponse, AppError> {
  let claims = req
    .extensions()
    .get::<Claims>()
    .cloned()
    .ok_or(AppError::Unauthorized)?;

  let result = {
    let mut orderbook = state.orderbook.lock().unwrap();
    orderbook.cancel_order(path.order_id, &claims.sub)
  };

  match result {
    Ok(_) => Ok(HttpResponse::Ok().json(ApiResponse::<()> {
      success: true,
      message: String::from("order deleted successfully"),
      data: None,
    })),
    Err(_) => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
      success: false,
      message: String::from("order not found"),
      data: None,
    })),
  }
}

pub async fn get_orders(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
  let orderbook = state.orderbook.lock().unwrap();
  Ok(HttpResponse::Ok().json(orderbook.depth()))
}
