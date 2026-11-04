use crate::helpers::hash::{hash_password, verify_password};
use crate::helpers::jwt::generate_token;
use crate::helpers::response::ApiResponse;
use crate::modules::auth::dto::{SigninRequest, SignupRequest};
use crate::{errors::AppError, state::AppState};
use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpResponse, web};
use solana_sdk::signer::{Signer, keypair::Keypair};

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
  let keypair = Keypair::new();
  println!("private key: {:?}", keypair.to_bytes());
  println!("public key: {:?}", keypair.pubkey());

  Ok(HttpResponse::Created().json(ApiResponse::<()> {
    success: true,
    message: String::from("Signup successful"),
    data: None,
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
    let token = generate_token(user.id.to_string(), &state.config.jwt_secret);
    let cookie = Cookie::build("access_token", token)
      .http_only(true)
      .secure(false)
      .same_site(SameSite::Lax)
      .path("/")
      .max_age(Duration::hours(24))
      .finish();
    Ok(HttpResponse::Ok().cookie(cookie).json(ApiResponse::<()> {
      message: String::from("Signin successful"),
      success: true,
      data: None,
    }))
  }
}
