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
