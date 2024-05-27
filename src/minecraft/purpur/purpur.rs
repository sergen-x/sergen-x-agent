use crate::common::http;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Versions {
    pub project: String,
    pub versions: Vec<String>,
}

// Projects = ["purpur, "purformance"]
pub async fn get_versions(project: &str) -> Result<Versions, Error> {
    let url = format!("https://api.purpurmc.org/v2/{project}");
    let versions: Versions = http::get(&url).await?;
    Ok(versions)
}

pub async fn download_version(
    project: &str,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.purpurmc.org/v2/{project}/{version}/latest/download"
    );
    http::download_file(&url).await;
    Ok(())
}
