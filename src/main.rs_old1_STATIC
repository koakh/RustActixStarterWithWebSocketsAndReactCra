// #[allow(unused_imports)]
use actix_web::{web, App, HttpServer};
use actix_web_static_files;
use log::debug;
use starter::constants::APP_NAME;
use starter::websockets::index;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || {
    debug!("starting app {}", APP_NAME);
    let generated = generate();
    App::new()
      // static
      .service(
        actix_web_static_files::ResourceFiles::new("/", generated).resolve_not_found_to_root(),
      )
      // webSockets
      .route("/ws/", web::get().to(index))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
