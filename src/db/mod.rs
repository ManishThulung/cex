use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::AppConfig;

pub async fn create_pool(config: &AppConfig) -> Result<PgPool, sqlx::Error> {
  PgPoolOptions::new()
    .max_connections(config.database_max_connections)
    .connect(&config.database_url)
    .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
  sqlx::migrate!("./migrations").run(pool).await
}
