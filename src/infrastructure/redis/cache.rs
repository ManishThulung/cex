use deadpool_redis::{Pool, redis::AsyncCommands};
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

#[derive(Clone)]
pub struct RedisService {
  pool: Pool,
}

#[derive(Debug, Error)]
pub enum RedisServiceError {
  #[error("Failed to acquire Redis connection")]
  Pool(#[from] deadpool_redis::PoolError),

  #[error("Redis operation failed")]
  Redis(#[from] deadpool_redis::redis::RedisError),

  #[error("Failed to serialize or deserialize Redis value")]
  Serialization(#[from] serde_json::Error),
}

impl RedisService {
  pub fn new(pool: Pool) -> Self {
    Self { pool }
  }

  pub async fn ping(&self) -> Result<(), RedisServiceError> {
    let mut conn = self.pool.get().await?;

    let _: String = deadpool_redis::redis::cmd("PING")
      .query_async(&mut conn)
      .await?;

    Ok(())
  }

  pub async fn set<T>(
    &self,
    key: &str,
    value: &T,
    ttl_seconds: u64,
  ) -> Result<(), RedisServiceError>
  where
    T: Serialize,
  {
    let mut conn = self.pool.get().await?;

    let serialized = serde_json::to_string(value)?;

    let _: () = conn.set_ex(key, serialized, ttl_seconds).await?;

    Ok(())
  }

  pub async fn get<T>(&self, key: &str) -> Result<Option<T>, RedisServiceError>
  where
    T: DeserializeOwned,
  {
    let mut conn = self.pool.get().await?;

    let value: Option<String> = conn.get(key).await?;

    match value {
      Some(value) => Ok(Some(serde_json::from_str(&value)?)),
      None => Ok(None),
    }
  }

  pub async fn delete(&self, key: &str) -> Result<(), RedisServiceError> {
    let mut conn = self.pool.get().await?;

    let _: u64 = conn.del(key).await?;

    Ok(())
  }

  pub async fn exists(&self, key: &str) -> Result<bool, RedisServiceError> {
    let mut conn = self.pool.get().await?;

    let exists: bool = conn.exists(key).await?;

    Ok(exists)
  }
}
