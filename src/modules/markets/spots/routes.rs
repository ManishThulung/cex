use crate::modules::markets::spots::handler::{create_spot, delete_spot, get_spot};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/markets/spots")
      .route("", web::post().to(create_spot))
      .route("", web::get().to(get_spot))
      // .route("/{id}", web::get().to(get_user))
      // .route("/{id}", web::put().to(update_user))
      .route("/{id}", web::delete().to(delete_spot)),
  );
}
