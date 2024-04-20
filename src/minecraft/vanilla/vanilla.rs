use serde::Deserialize;
use crate::common::http;
use crate::minecraft::vanilla::versions::VersionInfo;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
}

impl VersionManifest {
    pub fn find_url_by_id(&self, id: &str) -> Option<&str> {
        for version in &self.versions {
            if version.id == id {
                return Some(&version.url);
            }
        }
        None
    }
}

pub async fn get_all_versions() -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    let versions: VersionManifest = http::get(url).await?;
    Ok(versions)
}

pub async fn get_download_url(
    version: &str,
    version_manifest: VersionManifest,
) -> Result<Option<VersionInfo>, Box<dyn std::error::Error>> {
    match version_manifest.find_url_by_id(version) {
        Some(url) => {
            let info: VersionInfo = http::get(url).await?;
            Ok(Some(info))
        }
        None => {
            println!("Version {} could not located in the version manifest.", version);
            Ok(None)
        }
    }
}

pub async fn download_version(
    version_info: VersionInfo
) -> Result<(), Box<dyn std::error::Error>> {
    //let url = &version_manifest.latest.release;
    println!("{}", &version_info.downloads.server.url);
    http::download_file(&version_info.downloads.server.url).await?;
    Ok(())
}