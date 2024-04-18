use serde::Deserialize;
use crate::common::http;

// Documentation: https://github.com/adoptium/api.adoptium.net/blob/main/docs/cookbook.adoc
#[derive(Debug, Deserialize)]
struct AdoptiumVersions {
    #[serde(rename = "available_lts_releases")]
    lts_releases: Vec<i32>, 
    #[serde(rename = "available_releases")]
    _releases: Vec<i32>, 
    #[serde(rename = "most_recent_feature_release")]
    _latest_feature_release: i32,
    #[serde(rename = "most_recent_feature_version")]
    _latest_feature_version: i32, 
    #[serde(rename = "most_recent_lts")]
    _latest_lts: i32, 
    #[serde(rename = "tip_version")]
    _tip_version: i32, 
}

pub async fn get_versions() -> Result<(), Box<dyn std::error::Error>> {

    let _supported_os = vec![
        "linux",
        "windows",
        "macos",
        "solaris",
        "aix",
        "alpine-linux",
    ];
    
    let _supported_architecture = vec![
        "x64",
        "x86",
        "x32",
        "ppc64",
        "ppc64le",
        "alpine-s390x",
        "aarch64",
        "arm",
        "sparcv9",
        "riscv64",
    ];
    let _image_type = vec![
        "jdk",
        "jre"
    ];


    let url = "https://api.adoptium.net/v3/info/available_releases";
    let versions: AdoptiumVersions = http::get(url).await?;
    println!("{:?}", versions.lts_releases);

    Ok(())
}

pub async fn download_version(
    feature_version: &str,
    os: &str,
    arch: &str,
    image_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/{}/hotspot/normal/eclipse",
        feature_version, os, arch, image_type
    );
    println!("{}", url);
    http::download_file(&url).await?;
    println!("Downloaded");
    Ok(())
}