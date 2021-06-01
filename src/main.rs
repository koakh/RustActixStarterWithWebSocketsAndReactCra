#[macro_use]
extern crate log;

use actix::prelude::*;
use actix::Addr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files;
// use actix_rt;
// use actix_web_actors::ws;

use dotenv::dotenv;
use log::debug;
// use std::time::{Duration, Instant};
use serde_json::json;

mod constants;
mod dto;
mod websocket;
mod errors;

use constants::APP_NAME;
use dto::{PostInput, PostResponse};
use websocket::{ws_index, Server as WebServer, MessageToClient};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

// /// How often heartbeat pings are sent
// const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// /// How long before lack of client response causes a timeout
// const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

// /// do websocket handshake and start `WebSocketSession` actor
// async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//   println!("{:?}", r);
//   let res = ws::start(WebSocketSession::new(), &r, stream);
//   println!("{:?}", res);
//   res
// }

// /// websocket connection is long running connection, it easier
// /// to handle with an actor
// struct WebSocketSession {
//   /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
//   /// otherwise we drop connection.
//   hb: Instant,
// }

// impl Actor for WebSocketSession {
//   type Context = ws::WebsocketContext<Self>;
//   /// Method is called on actor start. We start the heartbeat process here.
//   fn started(&mut self, ctx: &mut Self::Context) {
//     self.hb(ctx);
//   }
// }

// /// Handler for `ws::Message`
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
//   fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//     // process websocket messages
//     println!("WS: {:?}", msg);
//     match msg {
//       Ok(ws::Message::Ping(msg)) => {
//         self.hb = Instant::now();
//         ctx.pong(&msg);
//       }
//       Ok(ws::Message::Pong(_)) => {
//         self.hb = Instant::now();
//       }
//       Ok(ws::Message::Text(text)) => ctx.text(text),
//       Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//       Ok(ws::Message::Close(reason)) => {
//         ctx.close(reason);
//         ctx.stop();
//       }
//       _ => ctx.stop(),
//     }
//   }
// }

// impl WebSocketSession {
//   fn new() -> Self {
//     Self { hb: Instant::now() }
//   }
//   /// helper method that sends ping to client every second.
//   ///
//   /// also this method checks heartbeats from client
//   fn hb(&self, ctx: &mut <Self as Actor>::Context) {
//     ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
//       // check client heartbeats
//       if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
//         // heartbeat timed out
//         println!("Websocket Client heartbeat failed, disconnecting!");
//         // stop actor
//         ctx.stop();
//         // don't try to send a ping
//         return;
//       }
//       ctx.ping(b"");
//     });
//   }
// }

#[get("/hello")]
async fn hello() -> impl Responder {
  // WebSocketSession::hb();
  HttpResponse::Ok().body("Hello world!")
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//   HttpResponse::Ok().body(req_body)
// }

#[post("/echo")]
async fn echo(
  msg: web::Json<PostInput>,
  websocket_srv: web::Data<Addr<WebServer>>,
) -> HttpResponse /*Result<web::Json<PostResponse>, Box<dyn std::error::Error>>*/ {
  // The type of `j` is `serde_json::Value`
  let json = json!({ "fingerprint": "0xF9BA143B95FF6D82" });
  // let wsm: WebSocketMessage = serde_json::from_value(json).unwrap();
  let message_to_client = MessageToClient::new("newquestion", json);
  // websocket_srv.do_send(message_to_client);
  match websocket_srv.send(message_to_client).await {
    Ok(ok) => {
      debug!("{:?}", ok);
    },
    Err(e) => {
      debug!("{:?}", e);
    }
  };
  HttpResponse::Ok().json(PostResponse {
    message: msg.message.clone(),
  })
}

// TODO
#[actix_web::main]
// TODO required actix-rt = "2.1.0"
// #[actix_rt::main]
// with actix_rt::main gives
// thread 'main' panicked at 'System is not running', /home/mario/.cargo/registry/src/github.com-1ecc6299db9ec823/actix-rt-1.1.1/src/system.rs:78:21
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info,debug");
  env_logger::init();
  debug!("starting app {}", APP_NAME);
  HttpServer::new(|| {
    let ws_server = WebServer::new().start();
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
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
