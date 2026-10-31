use crate::config::AppConfig;
use crate::modules::auth::service::AuthService;
use crate::modules::orders::model::Orderbook;
use crate::modules::users::service::UserService;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
  pub config: AppConfig,
  pub users: UserService,
  pub auth: AuthService,
  pub orderbook: Arc<Mutex<Orderbook>>,
}
