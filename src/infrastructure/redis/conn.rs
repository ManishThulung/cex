use deadpool_redis::{Config, Pool, Runtime};

pub type RedisPool = Pool;

pub fn create_redis_pool(
  redis_url: &str,
  max_size: usize,
) -> Result<RedisPool, deadpool_redis::CreatePoolError> {
  let mut config = Config::from_url(redis_url);

  config.pool = Some(deadpool_redis::PoolConfig {
    max_size,
    ..Default::default()
  });

  config.create_pool(Some(Runtime::Tokio1))
}
