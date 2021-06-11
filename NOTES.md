# NOTES

- [NOTES](#notes)
  - [Links](#links)
  - [install cargo-watch](#install-cargo-watch)
  - [Bootstrap](#bootstrap)
    - [Add actix-web-static-files` dependencies](#add-actix-web-static-files-dependencies)
    - [Add CRA App Javascript](#add-cra-app-javascript)
    - [Add CRA App TypeScript](#add-cra-app-typescript)
    - [Run and Test Embbeded Cra](#run-and-test-embbeded-cra)
    - [using AutoreLoad](#using-autoreload)
  - [Add WebSockets to Server Project](#add-websockets-to-server-project)
    - [Test Actix Sample](#test-actix-sample)
      - [Server](#server)
      - [Web Client](#web-client)
      - [Rust client](#rust-client)
    - [Add WebSockets to React Project](#add-websockets-to-react-project)
  - [Building a REST and Web Socket API with Actix and Rust / Best Tutorial with `.data(server.clone())](#building-a-rest-and-web-socket-api-with-actix-and-rust--best-tutorial-with-dataserverclone)
    - [Test WebSocket and use HotRealod with React Project](#test-websocket-and-use-hotrealod-with-react-project)
  - [Problems](#problems)
    - [Fix: the trait `Factory<_, _, _>` is not implemented for `fn(HttpRequest, actix_web::web::Payload, actix_web::web::Data<Addr<server::Server>>)](#fix-the-trait-factory_-_-_-is-not-implemented-for-fnhttprequest-actix_webwebpayload-actix_webwebdataaddrserverserver)
  - [the trait bound `WebSocketSession: actix::actor::Actor` is not satisfied](#the-trait-bound-websocketsession-actixactoractor-is-not-satisfied)
  - [Add WebSockets to Other Actix Web project like c3-updater](#add-websockets-to-other-actix-web-project-like-c3-updater)

## Links

- [How to create an API with Rust and Postgres - LogRocket Blog](https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/)

## install cargo-watch

```shell
$ cargo install systemfd cargo-watch
```

## Bootstrap

- [kilork/actix-web-static-files](https://github.com/kilork/actix-web-static-files)
- [actix_web_static_files - Rust](https://docs.rs/actix-web-static-files/3.0.5/actix_web_static_files/index.html?search=#use-case-3-packagejson---webpack-usage)
- [kilork/actix-web-static-files-example-angular-router](https://github.com/kilork/actix-web-static-files-example-angular-router)

### Add actix-web-static-files` dependencies

- [actix_web_static_files - Rust](https://docs.rs/actix-web-static-files/3.0.5/actix_web_static_files/index.html#actix-web-static-files-as-resources-support)

use `actix-web-static-files` dependencies

add to `Cargo.toml`

```toml
[package]
build = "build.rs"

[dependencies]
actix-web-static-files = "3.0"

[build-dependencies]
actix-web-static-files = "3.0"
```

add `build.rs`

```rust
// Use-case #1: Static resources folder
// use actix_web_static_files::resource_dir;
// fn main() {
//   // resource_dir("./static").build().unwrap();
//   resource_dir("./app/build").build().unwrap();
// }

// Use-case #3: package.json - WebPack usage
use actix_web_static_files::NpmBuild;
fn main() {
  NpmBuild::new("./app")
    .install()
    .unwrap()
    .run("build")
    .unwrap()
    .target("./app/build")
    .to_resource_dir()
    .build()
    .unwrap();
}
```

### Add CRA App Javascript

```shell
$ npx create-react-app app
```

### Add CRA App TypeScript

```shell
$ npx create-react-app --template cra-template-typescript app
```

### Run and Test Embbeded Cra

```shell
$ cargo build
$ cargo run
# run binary
$ cargo build
$ ./target/debug/rust-react-starter
# run binary release
$ cargo build --release
$ ./target/release/rust-react-starter
$ ls -lah ./target/release/rust-react-starter 
-rwxr-xr-x 2 mario users 8,4M mai 19 12:16 ./target/release/rust-react-starter
```

### using AutoreLoad

- [Auto-Reloading](https://actix.rs/docs/autoreload/)

```shell
$ cargo watch -x run
```

## Add WebSockets to Server Project

- [Websockets](https://actix.rs/docs/websockets/)

A simple websocket echo server example is available in the [examples directory](https://github.com/actix/examples/tree/master/websockets/websocket).

An example chat server with the ability to chat over a websocket or TCP connection is available in [websocket-chat directory](https://github.com/actix/examples/tree/master/websockets/chat)

### Test Actix Sample

- [actix/examples](https://github.com/actix/examples/tree/master/websockets/websocket)

#### Server

```shell
$ cd examples/websocket
$ cargo run --bin websocket-server
# Started http server: 127.0.0.1:8080
WS: Ok(Ping(b""))
WS: Ok(Ping(b""))
```

#### Web Client

- [http://localhost:8080/](http://localhost:8080/)

#### Rust client

```shell
$ cd examples/websocket
$ cargo run --bin websocket-client
ClientResponse HTTP/1.1 101 Switching Protocols
  headers:
    "date": "Wed, 19 May 2021 15:17:56 GMT"
    "upgrade": "websocket"
    "connection": "upgrade"
    "sec-websocket-accept": "UETwkO3imj9WAiFC/thWr1Tghl4="

Connected
```

1. copy example `main.rs`  to main project `main.rs` and start from that point
2. copy static folder from example
3. add cargo deps

```toml
[dependencies]
log = "0.4.0"
env_logger = "0.8.3"
actix = "0.10"
actix-web = "3"
actix-web-actors = "3"
# used in websocket sample only
actix-files = "0.5.0"
```

```shell
$ cargo run
```

test Web Client with [http://localhost:8080/](http://localhost:8080/)

### Add WebSockets to React Project

copy example `main.rs`  to main project `main.rs` and start from that point

- [actix/examples](https://github.com/actix/examples/tree/master/websockets/websocket)
- [WebSockets tutorial: How to go real-time with Node and React - LogRocket Blog](https://blog.logrocket.com/websockets-tutorial-how-to-go-real-time-with-node-and-react-8e4693fbf843/)
- [websocket](https://www.npmjs.com/package/websocket)

```shell
$ cd app/src
$ npm i websocket
$ npm i --save-dev @types/websocket
```

import { w3cwebsocket as W3CWebSocket } from "websocket";

use example **Client Example using the W3C WebSocket API** from [link](https://www.npmjs.com/package/websocket)

## Building a REST and Web Socket API with Actix and Rust / Best Tutorial with `.data(server.clone())

> follow Note Rust - **Actix WebSockets.md**

### Test WebSocket and use HotRealod with React Project

```shell
# terminal #1
$ make start_server
# terminal #2: use other outside frontend at port 8081 to use hor reload, this way we don't use embbedded version
$ make start_client
# minimal request
$ curl -X GET http://127.0.0.1:8080/hello
# open some frontend pages at http://127.0.0.1:8080|8081 and test websockets
$ curl -X POST -H "Content-Type: application/json" -d '{"message": "hello after clear...."}' http://127.0.0.1:8080/ws-echo | jq
```

## Problems

### Fix: the trait `Factory<_, _, _>` is not implemented for `fn(HttpRequest, actix_web::web::Payload, actix_web::web::Data<Addr<server::Server>>)

> the trick is adding src/errors.rs, after this error is gone :(
 
```shell
9 |     .service(web::resource("/ws/").route(web::get().to(websocket::ws_index)))
  |                                                        ^^^^^^^^^^^^^^^^^^^ the trait `Factory<_, _, _>` is not implemented for `fn(HttpRequest, actix_web::web::Payload, actix_web::web::Data<Addr<server::Server>>) -> impl std::future::Future {ws_index}`
```

## the trait bound `WebSocketSession: actix::actor::Actor` is not satisfied

- [The trait bound `Ws: actix::actor::Actor` is not satisfied - actix-web](https://www.gitmemory.com/issue/actix/actix-web/2121/808719620)

- [The trait bound `Ws: actix::actor::Actor` is not satisfied · Issue #2121 · actix/actix-web](https://github.com/actix/actix-web/issues/2121)

> above post has the answear: actix-web-actors 3.0 does not support actix 0.11. Use 0.10 instead

occurs when move code to c3-updater, and we up a dependencie `actix = "0.10.0"` to actix = "0.12.0" version

```shell
error[E0277]: the trait bound `WebSocketSession: actix::actor::Actor` is not satisfied
   --> src/websocket/mod.rs:87:74
    |
87  |   fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    |                                                                          ^^^^^^^^^^^^^^^^^^ the trait `actix::actor::Actor` is not implemented for `WebSocketSession`
```

- [Building a REST and Web Socket API with Actix and Rust](https://agmprojects.com/blog/building-a-rest-and-web-socket-api-with-actix.html)

We have a compiler error, similar to the one we have on the websocket Server.

```shell
the trait bound `websocket::WebSocketSession: actix::actor::Actor` is not satisfied
the trait `actix::actor::Actor` is not implemented for `websocket::WebSocketSession`
```

The fix for the Server was pretty straight forward, but this one is a little more involved.

## Add WebSockets to Other Actix Web project like c3-updater

wip