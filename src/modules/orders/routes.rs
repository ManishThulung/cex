use crate::middlewares::auth::auth_middleware;
use crate::modules::orders::handler::{create_order, delete_order, get_orders};
use actix_web::middleware::from_fn;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(
    // web::resource("/ws").route(web::get().to(ws::order_ws)),
    web::scope("/orders")
      // .route("/ws", web::get().to(ws::order_ws))
      .wrap(from_fn(auth_middleware))
      .route("", web::post().to(create_order))
      .route("", web::get().to(get_orders))
      .route("/{order_id}", web::delete().to(delete_order)),
  );
}
