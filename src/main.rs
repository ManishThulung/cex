use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, web};
use server::{
  config::AppConfig,
  db::create_pool,
  modules::{
    auth::{
      repository::AuthRepository, routes::configure as configure_auth_routes, service::AuthService,
    },
    orders::{model::Orderbook, routes::configure as configure_order_routes},
    users::{
      repository::UserRepository, routes::configure as configure_user_routes, service::UserService,
    },
  },
  state::AppState,
};
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  env_logger::init();

  let config = AppConfig::from_env().expect("Invalid application configuration");

  let pool = create_pool(&config)
    .await
    .expect("Failed to connect to PostgreSQL");

  // run_migrations(&pool)
  //   .await
  //   .expect("Failed to run database migrations");

  let users = UserService::new(UserRepository::new(pool.clone()));
  let auth = AuthService::new(AuthRepository::new(pool.clone()));
  let orderbook = Arc::new(Mutex::new(Orderbook::new()));

  let state = web::Data::new(AppState {
    users,
    auth,
    orderbook,
  });

  let host = config.host.clone();
  let port = config.port;

  log::info!("Starting server on {host}:{port}");

  HttpServer::new(move || {
    App::new()
      .app_data(state.clone())
      .wrap(Logger::default())
      .route(
        "/health",
        web::get().to(|| async {
          HttpResponse::Ok().json(serde_json::json!({
              "status": "ok"
          }))
        }),
      )
      .service(
        web::scope("/api/v1")
          .configure(configure_user_routes)
          .configure(configure_order_routes)
          .configure(configure_auth_routes),
      )
  })
  .bind((host, port))?
  .run()
  .await
}
