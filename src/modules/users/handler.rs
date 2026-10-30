use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::{errors::AppError, state::AppState};

use super::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};

pub async fn create_user(
  state: web::Data<AppState>,
  body: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, AppError> {
  let user = state.users.create(body.into_inner()).await?;

  Ok(HttpResponse::Created().json(UserResponse::from(user)))
}

pub async fn get_users(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
  let users = state.users.find_all().await?;

  let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

  Ok(HttpResponse::Ok().json(response))
}

pub async fn get_user(
  state: web::Data<AppState>,
  id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
  let user = state.users.find_by_id(id.into_inner()).await?;

  Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub async fn update_user(
  state: web::Data<AppState>,
  id: web::Path<Uuid>,
  body: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, AppError> {
  let user = state
    .users
    .update(id.into_inner(), body.into_inner())
    .await?;

  Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub async fn delete_user(
  state: web::Data<AppState>,
  id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
  state.users.delete(id.into_inner()).await?;

  Ok(HttpResponse::NoContent().finish())
}
