use super::model::User;
use crate::errors::AppError;
use crate::modules::auth::repository::AuthRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
  repository: AuthRepository,
}

impl AuthService {
  pub fn new(repository: AuthRepository) -> Self {
    Self { repository }
  }

  pub async fn create_user(
    &self,
    name: String,
    email: String,
    password: String,
  ) -> Result<User, AppError> {
    let (name, email) = validate_user(&name, &email)?;

    self
      .repository
      .create(Uuid::new_v4(), &name, &email, &password)
      .await
      .map_err(AppError::from)
  }

  pub async fn find_user(&self, email: String) -> Result<User, AppError> {
    self.repository.find_user(email).await.map_err(|error| {
      if matches!(error, sqlx::Error::RowNotFound) {
        AppError::NotFound
      } else {
        AppError::from(error)
      }
    })
  }

  //   pub async fn update(&self, id: Uuid, request: UpdateUserRequest) -> Result<User, AppError> {
  //     let (name, email) = validate_user(&request.name, &request.email)?;

  //     self
  //       .repository
  //       .update(id, &name, &email)
  //       .await
  //       .map_err(|error| {
  //         if matches!(error, sqlx::Error::RowNotFound) {
  //           AppError::NotFound
  //         } else {
  //           AppError::from(error)
  //         }
  //       })
  //   }

  //   pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
  //     let deleted = self.repository.delete(id).await?;

  //     if !deleted {
  //       return Err(AppError::NotFound);
  //     }

  //     Ok(())
  //   }
}

fn validate_user(name: &str, email: &str) -> Result<(String, String), AppError> {
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
