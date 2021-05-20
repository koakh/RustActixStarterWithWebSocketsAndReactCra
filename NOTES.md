# NOTES

- [How to create an API with Rust and Postgres - LogRocket Blog](https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/)- 

```shell
$ cargo run
$ curl -v http://localhost:8080/static/hello
Hello, world
```

## install cargo-watch

```shell
$ cargo install systemfd cargo-watch
```

## React Starter

## Links

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

### Add WebSockets to Server Project

- [websocket](https://www.npmjs.com/package/websocket)

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






https://docs.rs/actix_send_websocket/0.1.0/actix_send_websocket/
https://crates.io/crates/actix_send_websocket