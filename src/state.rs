use std::sync::{Arc, Mutex};

use crate::modules::auth::service::AuthService;
use crate::modules::{orders::model::Orderbook, users::service::UserService};

#[derive(Clone)]
pub struct AppState {
  pub users: UserService,
  pub auth: AuthService,
  pub orderbook: Arc<Mutex<Orderbook>>,
}
