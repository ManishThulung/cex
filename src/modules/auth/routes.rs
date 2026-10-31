use crate::modules::auth::handler::{signin, signup};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/auth")
      .route("/signin", web::post().to(signin))
      .route("/signup", web::post().to(signup)),
    // .route("/{id}", web::get().to(get_user))
    // .route("/{id}", web::put().to(update_user))
    // .route("/{id}", web::delete().to(delete_user)),
  );
}
