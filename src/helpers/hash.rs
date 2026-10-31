use argon2::{
  Argon2, PasswordHash,
  password_hash::{PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

/// Hash a plaintext password using Argon2id.
pub fn hash_password(password: String) -> Result<String, argon2::password_hash::Error> {
  let salt = SaltString::generate(&mut OsRng);

  let argon2 = Argon2::default();

  let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

  Ok(password_hash.to_string())
}

/// Verify a plaintext password against its stored hash.
pub fn verify_password(
  password: String,
  stored_hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
  let parsed_hash = PasswordHash::new(stored_hash)?;

  let argon2 = Argon2::default();

  Ok(
    argon2
      .verify_password(password.as_bytes(), &parsed_hash)
      .is_ok(),
  )
}
