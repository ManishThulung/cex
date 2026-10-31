use std::sync::{Arc, Mutex};

use crate::modules::{orders::model::Orderbook, users::service::UserService};

#[derive(Clone)]
pub struct AppState {
  pub users: UserService,
  pub orderbook: Arc<Mutex<Orderbook>>,
}
