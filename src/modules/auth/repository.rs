use sqlx::PgPool;
use uuid::Uuid;

use super::model::User;

#[derive(Clone)]
pub struct AuthRepository {
  pool: PgPool,
}

impl AuthRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }

  pub async fn create(
    &self,
    id: Uuid,
    name: &str,
    email: &str,
    password: &str,
  ) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
      r#"
            INSERT INTO users (id, name, email, password)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, password, created_at, updated_at
            "#,
    )
    .bind(id)
    .bind(name)
    .bind(email)
    .bind(password)
    .fetch_one(&self.pool)
    .await
  }

  pub async fn find_user(&self, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
      r#"
            SELECT id, email, password, name, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
    )
    .bind(email)
    .fetch_one(&self.pool)
    .await
  }

  pub async fn update(&self, id: Uuid, name: &str, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
      r#"
            UPDATE users
            SET name = $1,
                email = $2,
                updated_at = NOW()
            WHERE id = $3
            RETURNING id, name, email, created_at, updated_at
            "#,
    )
    .bind(name)
    .bind(email)
    .bind(id)
    .fetch_one(&self.pool)
    .await
  }

  pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
      .bind(id)
      .execute(&self.pool)
      .await?;

    Ok(result.rows_affected() > 0)
  }
}
