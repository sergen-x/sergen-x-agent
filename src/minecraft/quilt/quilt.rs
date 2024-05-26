use crate::common::http;
use crate::common::kv::pair::Map;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct VersionResponse {
    versions: Map<String, Details>,
}

#[derive(Deserialize)]
pub struct Details {
    pub quilt_loader: String,
    pub loom: String,
    pub quilt_mappings: String,
    pub quilted_fabric_api: String,
}

pub async fn download_installer() -> Result<(), Box<dyn Error>> {
    http::download_file(
        "https://quiltmc.org/api/v1/download-latest-installer/java-universal",
    )
    .await?;
    Ok(())
}

pub async fn get_installer_version() -> Result<VersionResponse, Box<dyn Error>>
{
    let url = "https://quiltmc.org/api/v1/latest-version-components";
    let res: VersionResponse = http::get(&url).await?;
    Ok(res)
}

pub fn run_installer(
    version: &str,
    installer_version: &str,
) {
    let command = format!(
        "java -jar
        quilt-installer-{installer_version}.jar
        install server {version}
        --download-server"
    );
}
