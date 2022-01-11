#[macro_use]
extern crate log;

use actix::prelude::*;
use actix::Addr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web_static_files;
use dotenv::dotenv;
use log::{debug, error, info};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde_json::json;
use std::cell::Cell;
use std::env;
// use std::sync::Arc;
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::Mutex;
use std::sync::{
  atomic::{AtomicUsize, Ordering},
  Arc, Mutex,
};
use std::time::SystemTime;
use time::OffsetDateTime;
use uuid::Uuid;

mod app;
mod dto;
mod types;
mod util;
mod websocket;

use app::{APP_NAME, CONFIG_PATH_SSL, DATE_FORMAT_STR, DEFAULT_HTTPS_SERVER_URI};
use dto::{PostWsEchoRequest, PostWsEchoResponse};
use types::MessageToClientType;
use util::out_message;
use websocket::{ws_index, MessageToClient, Server as WebServer};

use crate::app::{AppState, AppStateGlobal, AppStateResponse, PostStateRequest};

// for static files
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn current_formatted_date(date_format_str: &str) -> String {
  let dt: OffsetDateTime = SystemTime::now().into();
  dt.format(date_format_str)
}

#[get("/hello")]
async fn hello() -> impl Responder {
  // WebSocketSession::hb();
  HttpResponse::Ok().body("Hello world!")
}

#[post("/state")]
async fn state(msg: web::Json<PostStateRequest>, data: web::Data<AppState>, app_data: web::Data<AppStateGlobal>) -> Result<web::Json<AppStateResponse>> {
  // global get counter's MutexGuard
  let mut counter = app_data.counter.lock().unwrap();
  // access counter inside MutexGuard
  *counter += 1;

  if !msg.message.eq("") {
    let mut message = app_data.message.lock().unwrap();
    // access filter inside MutexGuard
    *message = msg.message.clone();
  }

  // workers state
  let request_count = data.request_count.get() + 1;
  data.request_count.set(request_count);

  debug!("{:?}", msg);

  Ok(web::Json(AppStateResponse {
    server_id: data.server_id,
    request_count,
    counter: *counter,
    message: String::from(&msg.message),
  }))
}

#[post("/ws-echo")]
async fn ws_echo(msg: web::Json<PostWsEchoRequest>, websocket_srv: web::Data<Addr<WebServer>>) -> HttpResponse /*Result<web::Json<PostWsEchoResponse>, Box<dyn std::error::Error>>*/ {
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
  HttpResponse::Ok().json(PostWsEchoResponse { message: msg.message.clone() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,debug,warn,error");
  env_logger::init();
  // environment variables
  // let bind_addr = env::var("BIND_ADDR").unwrap_or(DEFAULT_HTTP_SERVER_URI.to_string());
  // environment variables
  let http_server_uri = env::var("HTTP_SERVER_URI").unwrap_or(DEFAULT_HTTPS_SERVER_URI.to_string());
  out_message(format!("server start at: '{}'", http_server_uri), 0);

  // config https ssl keys
  let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
  builder.set_private_key_file(format!("{}/key.pem", CONFIG_PATH_SSL), SslFiletype::PEM).unwrap();
  builder.set_certificate_chain_file(format!("{}/cert.pem", CONFIG_PATH_SSL)).unwrap();

  // the trick for not lost connections sessions, is create ws_server outside of HttpServer::new, and use `move ||`
  let ws_server = WebServer::new().start();

  // init global data
  let data = web::Data::new(AppStateGlobal {
    counter: Mutex::new(0),
    message: Arc::new(Mutex::new(String::from(""))),
  });

  // bootstrap actix server
  info!("start app: {}", APP_NAME);

  HttpServer::new(move || {
    // init actix_web_static_files generated
    let generated = generate();
    App::new()
      // worker/thread data
      .data(AppState {
        server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
        request_count: Cell::new(0),
        // filter,
      })
      // global data
      .app_data(data.clone())
      // inject ws_server in context
      .data(ws_server.clone())
      // webSockets: TRICK /ws/ route must be before / and others to prevent problems
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      // static files: uncomment  to open static page on browser, comment and leave page open on browser for tests
      // .service(fs::Files::new("/", "static/").index_file("index.html"))
      // services
      .service(hello)
      .service(ws_echo)
      .service(state)
      // static, leave / route to the end, else it overrides all others
      .service(actix_web_static_files::ResourceFiles::new("/", generated).resolve_not_found_to_root())
  })
  // .bind(bind_addr)?
  // .bind(http_server_uri)?
  .bind_openssl(http_server_uri, builder)?
  .run()
  .await
}
