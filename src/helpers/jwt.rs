use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
  exp: usize,
  pub sub: String,
}

pub fn generate_token(user_id: String, jwt_secret: &str) -> String {
  let claims = Claims {
    sub: user_id,
    exp: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs() as usize
      + 60 * 60 * 24,
  };

  let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(jwt_secret.as_bytes()),
  )
  .unwrap();
  token
}

pub fn verify_token(token: &str, jwt_secret: String) -> Result<String, String> {
  match decode(
    token,
    &DecodingKey::from_secret(jwt_secret.as_bytes()),
    &Validation::default(),
  ) {
    Err(e) => Err(e.to_string()),
    Ok(token_data) => {
      let claims: Claims = token_data.claims;
      Ok(claims.sub)
    }
  }
}
