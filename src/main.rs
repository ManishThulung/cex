use actix_web::{App, HttpResponse, HttpServer, middleware::Logger, web};

use server::{
  config::AppConfig,
  db::{create_pool, run_migrations},
  modules::{
    orders::routes::configure as configure_order_routes,
    users::{
      repository::UserRepository, routes::configure as configure_user_routes, service::UserService,
    },
  },
  state::AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenvy::dotenv().ok();

  env_logger::init();

  let config = AppConfig::from_env().expect("Invalid application configuration");

  let pool = create_pool(&config)
    .await
    .expect("Failed to connect to PostgreSQL");

  run_migrations(&pool)
    .await
    .expect("Failed to run database migrations");

  let users = UserService::new(UserRepository::new(pool));

  let state = web::Data::new(AppState { users });

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
          .configure(configure_order_routes),
      )
  })
  .bind((host, port))?
  .run()
  .await
}
