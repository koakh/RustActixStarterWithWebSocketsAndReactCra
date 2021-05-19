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

## Add actix-web-static-files` dependencies

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
use actix_web_static_files::{resource_dir, NpmBuild};

// Use-case #1: Static resources folder
// fn main() {
//   // resource_dir("./static").build().unwrap();
//   resource_dir("./app/build").build().unwrap();
// }

// Use-case #3: package.json - WebPack usage
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

## Add CRA App

```shell
$ npx create-react-app app
```

## Run and Test Embbeded Cra

```shell
$ cargo build
$ cargo run
# run binary
$ cargo build
$ ./target/debug/rust-react-starter
# run binary release
$ cargo build --release
$ ./target/release/rust-react-starter
```
