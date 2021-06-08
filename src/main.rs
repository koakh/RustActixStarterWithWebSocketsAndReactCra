#[macro_use]
extern crate log;

use actix::prelude::*;
use actix::Addr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files;
use dotenv::dotenv;
use log::{debug, error, info};
use serde_json::json;
use std::env;
use std::time::SystemTime;
use time::OffsetDateTime;
use uuid::Uuid;

mod app;
mod dto;
mod types;
mod websocket;

use app::{APP_NAME, DATE_FORMAT_STR, DEFAULT_BIND_ADDR};
use dto::{PostInput, PostResponse};
use types::MessageToClientType;
use websocket::{ws_index, MessageToClient, Server as WebServer};

// for static files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub fn current_formatted_date(date_format_str: &str) -> String {
  let dt: OffsetDateTime = SystemTime::now().into();
  dt.format(date_format_str)
}

#[get("/hello")]
async fn hello() -> impl Responder {
  // WebSocketSession::hb();
  HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(
  msg: web::Json<PostInput>,
  websocket_srv: web::Data<Addr<WebServer>>,
) -> HttpResponse /*Result<web::Json<PostResponse>, Box<dyn std::error::Error>>*/ {
  // The type of `j` is `serde_json::Value`
  let json = json!({ "fingerprint": "0xF9BA143B95FF6D82", "date": current_formatted_date(DATE_FORMAT_STR), "uuid": Uuid::new_v4().to_string() });
  // let wsm: WebSocketMessage = serde_json::from_value(json).unwrap();
  let msg_type = &format!("{}", MessageToClientType::Echo)[..];
  let message_to_client = MessageToClient::new(msg_type, json);
  // let message_to_client = MessageToClient::new("echo", json);
  // websocket_srv.do_send(message_to_client);
  match websocket_srv.send(message_to_client).await {
    Ok(ok) => debug!("{:?}", ok),
    Err(e) => error!("{:?}", e),
  };
  HttpResponse::Ok().json(PostResponse {
    message: msg.message.clone(),
  })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  std::env::set_var(
    "RUST_LOG",
    "actix_server=info,actix_web=info,debug,warn,error",
  );
  env_logger::init();
  // environment variables
  let bind_addr = env::var("BIND_ADDR").unwrap_or(DEFAULT_BIND_ADDR.to_string());
  // bootstrap actix server
  info!("start app: {}", APP_NAME);

  // the trick for not lost connections sessions, is create ws_server outside of HttpServer::new, and use `move ||`
  let ws_server = WebServer::new().start();
  HttpServer::new(move || {
    let generated = generate();
    App::new()
      .data(ws_server.clone())
      // webSockets: TRICK /ws/ route must be before / and others to prevent problems
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      // static files: uncomment  to open static page on browser, comment and leave page open on browser for tests
      // .service(fs::Files::new("/", "static/").index_file("index.html"))
      // services
      .service(hello)
      .service(echo)
      // static, leave / route to the end, else it overrides all others
      .service(
        actix_web_static_files::ResourceFiles::new("/", generated).resolve_not_found_to_root(),
      )
  })
  .bind(bind_addr)?
  .run()
  .await
}
