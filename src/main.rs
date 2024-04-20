mod minecraft;
pub mod common;
pub mod dependencies;

#[tokio::main]
async fn main() {
    // let _ = minecraft::forge::versions::get_versions();
    // let _ = dependencies::java::adoptium::main::get_versions();
    //let _ = dependencies::java::adoptium::main::download_version("21", "linux", "x64", "jdk").await;
   
    let manifest = minecraft::vanilla::vanilla::get_all_versions().await.expect("Failed to fetch version manifest");
    let release_version = manifest.latest.release.clone();
    let version = minecraft::vanilla::vanilla::get_download_url(&release_version, manifest)
        .await
        .expect("Failed to fetch download URL");


    if let Some(version_info) = version {
        let _ = minecraft::vanilla::vanilla::download_version(version_info)
            .await
            .expect("Failed to download version");
    } else {
        panic!("VersionInfo is None, cannot download version");
    }
}