use sqlx::PgPool;
use uuid::Uuid;

use crate::modules::markets::spots::model::{OrderbookStatus, Spot};

#[derive(Clone)]
pub struct SpotRepository {
  pool: PgPool,
}

impl SpotRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn create(
    &self,
    id: Uuid,
    name: &str,
    symbol: &str,
    image_url: &str,
    base_symbol: &str,
    quote_symbol: &str,
    is_visible: bool,
    status: OrderbookStatus,
  ) -> Result<Spot, sqlx::Error> {
    sqlx::query_as::<_, Spot>(
      r#"
        INSERT INTO spots (id, name, symbol, image_url, base_symbol, quote_symbol, is_visible, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, name, symbol, image_url, base_symbol, quote_symbol, is_visible, status
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(symbol)
    .bind(image_url)
    .bind(base_symbol)
    .bind(quote_symbol)
    .bind(is_visible)
    .bind(status)       
    .fetch_one(&self.pool)
    .await
  }

  pub async fn find_all(&self) -> Result<Vec<Spot>, sqlx::Error> {
    sqlx::query_as::<_, Spot>(
      r#"
            SELECT id, name, symbol, image_url, base_symbol, quote_symbol, is_visible, status, created_at, updated_at
            FROM spots
            ORDER BY created_at DESC, id DESC
            LIMIT 100
            "#,
    )
    .fetch_all(&self.pool)
    .await
  }

  pub async fn find_by_id(&self, id: Uuid) -> Result<Spot, sqlx::Error> {
    sqlx::query_as::<_, Spot>(
      r#"
            SELECT id, name, symbol, image_url, base_symbol, quote_symbol, is_visible, status, created_at, updated_at
            FROM spots
            WHERE id = $1
            "#,
    )
    .bind(id)
    .fetch_one(&self.pool)
    .await
  }

  pub async fn update(&self, id: Uuid, name: &str, symbol: &str, image_url: &str, base_symbol: &str, quote_symbol: &str, is_visible: bool, status: OrderbookStatus  ) -> Result<Spot, sqlx::Error> {
    sqlx::query_as::<_, Spot>(
      r#"
            UPDATE spots
            SET name = $1,
                symbol = $2,
                image_url = $3,
                base_symbol = $4,
                quote_symbol = $5,
                is_visible = $6,
                status = $7,
                updated_at = NOW()
            WHERE id = $8
            RETURNING id, name, symbol, image_url, base_symbol, quote_symbol, is_visible, status, created_at, updated_at
            "#,
    )
    .bind(name)
    .bind(symbol)
    .bind(image_url)
    .bind(base_symbol)
    .bind(quote_symbol)
    .bind(is_visible)
    .bind(status)
    .bind(id)
    .fetch_one(&self.pool)
    .await
  }

  pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM spots WHERE id = $1")
      .bind(id)
      .execute(&self.pool)
      .await?;

    Ok(result.rows_affected() > 0)
  }
}
