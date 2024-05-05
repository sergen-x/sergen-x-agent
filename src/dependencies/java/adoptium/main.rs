use serde::Deserialize;
use crate::common::http;
use crate::common::installer::InstallerFuture;
use crate::common::sysinfo::systeminfo::SystemInfo;
use crate::dependencies::java::main::JavaDistribution;

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

pub fn get_architecture(system_info: SystemInfo) -> Result<String, Box<dyn std::error::Error>> {
    let architecture_type = match system_info.architecture.as_str() {
        "x86_64" => "x64",
        "x86" => "x86",
        "i386" | "i486" | "i586" | "i686" => "x32",
        "powerpc64" => "ppc64",
        "powerpc64le" => "ppc64le",
        "s390x" => "alpine-s390x",
        "aarch64" => "aarch64",
        "arm" => "arm",
        "sparcv9" => "sparcv9",
        "riscv64" => "riscv64",
        _ => return Err(format!("Unknown system architecture: {}", system_info.architecture).into()),
    };
    Ok(architecture_type.to_string())
}

pub struct Adoptium;
impl JavaDistribution for Adoptium {
    fn get_versions(&self) -> InstallerFuture {
        let _supported_os = vec![
            "linux",
            "windows",
            "macos",
            "solaris",
            "aix",
            "alpine-linux",
        ];
        let _image_type = vec![
            "jdk",
            "jre"
        ];

        let url = "https://api.adoptium.net/v3/info/available_releases";

        Box::pin(async move {
            let versions: AdoptiumVersions = http::get(url).await?;
            println!("{:?}", versions.lts_releases);
            Ok(())
        })
    }

    fn download_version(
        &self,
        feature_version: &str,
        system_info: SystemInfo,
        image_type: &str,
    ) -> InstallerFuture {
        let architecture = get_architecture(system_info.clone()).unwrap();
        let url = format!(
            "https://api.adoptium.net/v3/binary/latest/{}/ga/{}/{}/{}/hotspot/normal/eclipse",
            feature_version, system_info.os, architecture, image_type
        );
        Box::pin(async move {
            http::download_file(&url).await?;
            println!("Downloaded");
            Ok(())
        })
    }
}
