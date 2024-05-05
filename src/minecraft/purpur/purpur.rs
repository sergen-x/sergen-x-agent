use reqwest::Error;
use serde::{Deserialize, Serialize};
use crate::common::http;
use crate::common::installer::InstallerFuture;

#[derive(Serialize, Deserialize)]
struct Versions {
    pub project: String,
    pub versions: Vec<String>,
}

// Projects = ["purpur, "purformance"]
pub async fn get_versions(
    project: &str
) -> Result<Versions, Error> {
    let url = format!(
         "https://api.purpurmc.org/v2/{project}"
    );
    let versions: Versions = http::get(&url).await?;
    Ok(versions)
}

pub fn download_version(
    project: &str,
    version: &str,
) -> InstallerFuture {
    let url = format!(
        "https://api.purpurmc.org/v2/{project}/{version}/latest/download"
    );
    Box::pin(async move {
        http::download_file(&url);
        Ok(())
    })
}