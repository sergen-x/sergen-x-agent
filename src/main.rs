mod minecraft;
pub mod common;
pub mod dependencies;

#[tokio::main]
async fn main() {
    // let _ = minecraft::forge::versions::get_versions();
    // let _ = dependencies::java::adoptium::main::get_versions();
    let _ = dependencies::java::adoptium::main::download_version("21", "linux", "x64", "jdk").await;
}