use uuid::Uuid;

use crate::errors::AppError;

use super::{
  dto::{CreateUserRequest, UpdateUserRequest},
  model::User,
  repository::UserRepository,
};

#[derive(Clone)]
pub struct UserService {
  repository: UserRepository,
}

impl UserService {
  pub fn new(repository: UserRepository) -> Self {
    Self { repository }
  }

  pub async fn create(&self, request: CreateUserRequest) -> Result<User, AppError> {
    let (name, email) = validate_user(&request.name, &request.email)?;

    self
      .repository
      .create(Uuid::new_v4(), &name, &email, &request.password)
      .await
      .map_err(AppError::from)
  }

  pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
    Ok(self.repository.find_all().await?)
  }

  pub async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
    self.repository.find_by_id(id).await.map_err(|error| {
      if matches!(error, sqlx::Error::RowNotFound) {
        AppError::NotFound
      } else {
        AppError::from(error)
      }
    })
  }

  pub async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<User, AppError> {
    let (name, email) = validate_user(&request.name, &request.email)?;

    self
      .repository
      .update(id, &name, &email)
      .await
      .map_err(|error| {
        if matches!(error, sqlx::Error::RowNotFound) {
          AppError::NotFound
        } else {
          AppError::from(error)
        }
      })
  }

  pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
    let deleted = self.repository.delete(id).await?;

    if !deleted {
      return Err(AppError::NotFound);
    }

    Ok(())
  }
}

fn validate_user(name: &str, email: &str) -> Result<(String, String), AppError> {
  let name = name.trim();
  let email = email.trim();

  if name.is_empty() || name.chars().count() > 100 {
    return Err(AppError::Validation(
      "Name must contain 1-100 characters".to_string(),
    ));
  }

  if email.is_empty()
    || email.len() > 255
    || !email.contains('@')
    || email.chars().any(char::is_whitespace)
  {
    return Err(AppError::Validation("Invalid email address".to_string()));
  }

  Ok((name.to_string(), email.to_string()))
}
