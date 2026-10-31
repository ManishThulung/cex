use actix_web::{
  Error, HttpMessage,
  body::MessageBody,
  dev::{ServiceRequest, ServiceResponse},
  error::ErrorUnauthorized,
  middleware::Next,
  web,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{helpers::jwt::Claims, state::AppState};

pub async fn auth_middleware(
  req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  let state = req
    .app_data::<web::Data<AppState>>()
    .ok_or_else(|| ErrorUnauthorized("Application state unavailable"))?;

  let token = req
    .cookie("access_token")
    .map(|cookie| cookie.value().to_owned())
    .ok_or_else(|| ErrorUnauthorized("Authentication required"))?;

  let token_data = decode::<Claims>(
    &token,
    &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    &Validation::default(),
  )
  .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

  // Attach claims to request extensions
  req.extensions_mut().insert(token_data.claims);

  next.call(req).await
}
