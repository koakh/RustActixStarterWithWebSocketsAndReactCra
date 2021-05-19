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

## Add WebSockets to Project


