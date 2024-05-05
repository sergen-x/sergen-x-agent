use reqwest::Error;
use serde::Deserialize;
use crate::common::http;
use crate::common::installer::InstallerFuture;

#[derive(Deserialize)]
pub struct Versions {
    pub is_snapshot: bool,
    pub versions: Vec<String>,
}

pub async fn get_versions() -> Result<Versions, Error> {
    let url = "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge";
    let res: Versions = http::get(url).await?;
    Ok(res)
}

pub fn download_version(version: &str) -> InstallerFuture {
    let url = format!(
        "https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar"
    );
    Box::pin(async move {
        http::download_file(&url);
        Ok(())
    })
}