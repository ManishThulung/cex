use super::dto::UserResponse;
use crate::helpers::hash::{hash_password, verify_password};
use crate::helpers::jwt::generate_token;
use crate::helpers::response::ApiResponse;
use crate::modules::auth::dto::{SigninRequest, SignupRequest};
use crate::{errors::AppError, state::AppState};
use actix_web::{HttpResponse, web};

pub async fn signup(
  state: web::Data<AppState>,
  body: web::Json<SignupRequest>,
) -> Result<HttpResponse, AppError> {
  let body = body.into_inner();
  let hashed_password = hash_password(body.password).map_err(|_e| AppError::Conflict)?;
  state
    .auth
    .create_user(body.name, body.email, hashed_password)
    .await?;

  Ok(HttpResponse::Created().json(ApiResponse {
    success: true,
    message: String::from("Signup successful"),
    data: (),
  }))
}

pub async fn signin(
  state: web::Data<AppState>,
  body: web::Json<SigninRequest>,
) -> Result<HttpResponse, AppError> {
  let body = body.into_inner();
  let user = state.auth.find_user(body.email).await?;
  let is_verifed =
    verify_password(body.password, &user.password).map_err(|_e| AppError::Conflict)?;
  if !is_verifed {
    Err(AppError::InvalidCredentials)
  } else {
    let token = generate_token(user.id.to_string());
    Ok(HttpResponse::Ok().json(ApiResponse {
      message: String::from("Signin successful"),
      success: true,
      data: (token),
    }))
  }
}

// pub async fn update_user(
//   state: web::Data<AppState>,
//   id: web::Path<Uuid>,
//   body: web::Json<UpdateUserRequest>,
// ) -> Result<HttpResponse, AppError> {
//   let user = state
//     .users
//     .update(id.into_inner(), body.into_inner())
//     .await?;

//   Ok(HttpResponse::Ok().json(UserResponse::from(user)))
// }

// pub async fn delete_user(
//   state: web::Data<AppState>,
//   id: web::Path<Uuid>,
// ) -> Result<HttpResponse, AppError> {
//   state.users.delete(id.into_inner()).await?;

//   Ok(HttpResponse::NoContent().finish())
// }
