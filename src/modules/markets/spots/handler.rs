use crate::{
  errors::AppError, helpers::response::ApiResponse,
  modules::markets::spots::dto::CreateSpotMarketRequest, state::AppState,
};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn create_spot(
  state: web::Data<AppState>,
  body: web::Json<CreateSpotMarketRequest>,
) -> Result<HttpResponse, AppError> {
  // let spot = state.spots.create(body.into_inner()).await?;

  Ok(HttpResponse::Created().json(ApiResponse::<()> {
    success: true,
    message: String::from("Spot market created successfully"),
    data: None,
  }))
}

// pub async fn get_spots(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
//   let spots = state.spots.find_all().await?;

//   let response: Vec<ApiResponse> = spots.into_iter().map(ApiResponse<()>{
//     success: true,
//     message: String::from("Spot market created successfully"),
//     data: None,
//   }ollect();

//   Ok(HttpResponse::Ok().json(response))
// }

pub async fn get_spot(
  state: web::Data<AppState>,
  id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
  // let spot = state.spots.find_by_id(id.into_inner()).await?;

  Ok(HttpResponse::Ok().json(ApiResponse::<()> {
    success: true,
    message: String::from("Spot market created successfully"),
    data: None,
  }))
}

// pub async fn update_spot(
//   state: web::Data<AppState>,
//   id: web::Path<Uuid>,
//   body: web::Json<UpdateSpotMarketRequest>,
// ) -> Result<HttpResponse, AppError> {
//   let spot = state
//     .spots
//     .update(id.into_inner(), body.into_inner())
//     .await?;

//   Ok(HttpResponse::Ok().json(ApiResponse::<()> {
//     success: true,
//     message: String::from("Spot market created successfully"),
//     data: None,
//   }))
// }

pub async fn delete_spot(
  state: web::Data<AppState>,
  id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
  state.spots.delete(id.into_inner()).await?;

  Ok(HttpResponse::NoContent().finish())
}
