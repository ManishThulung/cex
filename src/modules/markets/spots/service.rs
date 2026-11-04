use super::repository::SpotRepository;
use crate::{
  errors::AppError,
  modules::markets::spots::{dto::CreateSpotMarketRequest, model::Spot},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct SpotService {
  repository: SpotRepository,
}

impl SpotService {
  pub fn new(repository: SpotRepository) -> Self {
    Self { repository }
  }

  pub async fn create(&self, request: CreateSpotMarketRequest) -> Result<Spot, AppError> {
    self
      .repository
      .create(
        Uuid::new_v4(),
        &request.name,
        &request.symbol,
        &request.image_url,
        &request.base_symbol,
        &request.quote_symbol,
        request.is_visible,
        request.status,
      )
      .await
      .map_err(AppError::from)
  }

  pub async fn find_all(&self) -> Result<Vec<Spot>, AppError> {
    Ok(self.repository.find_all().await?)
  }

  pub async fn find_by_id(&self, id: Uuid) -> Result<Spot, AppError> {
    self.repository.find_by_id(id).await.map_err(|error| {
      if matches!(error, sqlx::Error::RowNotFound) {
        AppError::NotFound
      } else {
        AppError::from(error)
      }
    })
  }

  pub async fn update(&self, id: Uuid, request: CreateSpotMarketRequest) -> Result<Spot, AppError> {
    self
      .repository
      .update(
        id,
        &request.name,
        &request.symbol,
        &request.image_url,
        &request.base_symbol,
        &request.quote_symbol,
        request.is_visible,
        request.status,
      )
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
