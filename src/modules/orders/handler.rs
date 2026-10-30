use actix_web::{HttpResponse, web};

use crate::{
  errors::AppError,
  modules::orders::dto::{CreateOrderRequest, CreateOrderResponse},
  state::AppState,
};

pub async fn create_order(
  // state: web::Data<AppState>,
  body: web::Json<CreateOrderRequest>,
) -> Result<HttpResponse, AppError> {
  println!("hi reaching here: {:?}", body);

  Ok(HttpResponse::Ok().json(CreateOrderResponse {
    success: true,
    message: String::from("value"),
  }))
}

pub async fn get_orders() -> Result<HttpResponse, AppError> {
  println!("hi reaching get orders");

  Ok(HttpResponse::Ok().json(CreateOrderResponse {
    success: true,
    message: String::from("value"),
  }))
}
