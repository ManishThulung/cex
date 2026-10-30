use crate::modules::users::service::UserService;

#[derive(Clone)]
pub struct AppState {
  pub users: UserService,
}
