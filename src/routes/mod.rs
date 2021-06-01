use actix_web::web;

use crate::websocket;

pub mod questions;

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::resource("/ws/").route(web::get().to(websocket::ws_index)))
    .service(
      web::scope("/api")
        .service(web::scope("/questions").route("", web::post().to(questions::create))),
    );
}
