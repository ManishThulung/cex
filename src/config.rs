use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
  pub host: String,
  pub port: u16,
  pub database_url: String,
  pub database_max_connections: u32,
  pub jwt_secret: String,

  pub redis_url: String,
  pub redis_pool_max_size: usize,
}

impl AppConfig {
  pub fn from_env() -> Result<Self, String> {
    let database_url =
      env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is required".to_string())?;

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let port = env::var("PORT")
      .unwrap_or_else(|_| "8080".to_string())
      .parse::<u16>()
      .map_err(|_| "PORT must be a valid u16".to_string())?;

    let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
      .unwrap_or_else(|_| "10".to_string())
      .parse::<u32>()
      .map_err(|_| "DATABASE_MAX_CONNECTIONS must be a positive integer".to_string())?;

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "test-secret".to_string());

    if database_max_connections == 0 {
      return Err("DATABASE_MAX_CONNECTIONS must be greater than zero".to_string());
    }

    let redis_url = env::var("REDIS_URL").map_err(|_| "REDIS_URL is required".to_string())?;
    let redis_pool_max_size = env::var("REDIS_POOL_MAX_SIZE")
      .unwrap_or_else(|_| "16".to_string())
      .parse::<usize>()
      .map_err(|_| "REDIS_POOL_MAX_SIZE must be a positive integer".to_string())?;
    if redis_pool_max_size == 0 {
      return Err("REDIS_POOL_MAX_SIZE must be greater than zero".to_string());
    }

    Ok(Self {
      host,
      port,
      database_url,
      database_max_connections,
      jwt_secret,
      redis_url,
      redis_pool_max_size,
    })
  }
}
