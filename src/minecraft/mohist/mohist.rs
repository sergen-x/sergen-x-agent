use std::error::Error;
use std::io;
use serde::Deserialize;
use crate::common::http;
use crate::common::installer::InstallerFuture;

#[derive(Deserialize)]
pub struct Versions {
    versions: Vec<String>
}

#[derive(Deserialize)]
struct Builds {
    #[serde(rename = "projectName")]
    pub project_name: String,
    #[serde(rename = "projectVersion")]
    pub project_version: String,
    pub builds: Vec<Build>,
}

#[derive(Deserialize, Clone)]
struct Build {
    pub number: i64,
    #[serde(rename = "gitSha")]
    pub git_sha: String,
    #[serde(rename = "forgeVersion")]
    pub forge_version: String,
    #[serde(rename = "fileMd5")]
    pub file_md5: String,
    #[serde(rename = "originUrl")]
    pub origin_url: String,
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

// Projects = ["mohist", "banner"]
pub async fn get_versions(
    project: &str,
) -> Result<Versions, Box<dyn Error>> {
    let url = format!(
        "https://mohistmc.com/api/v2/projects/{project}"
    );
    let versions: Versions = http::get(&url).await?;
    Ok(versions)
}

pub async fn get_builds(
    project: &str,
    version: &str,
) -> Result<Builds, Box<dyn Error>> {
    let url = format!(
        "https://mohistmc.com/api/v2/projects/{project}/{version}/builds"
    );
    let builds: Builds = http::get(&url).await?;
    Ok(builds)
}

impl Builds {
    pub fn download_latest(&self) -> InstallerFuture {
        let builds = self.builds.clone();
        Box::pin(async move {
            if let Some(last_build) = builds.last() {
                http::download_file(&last_build.url);
                Ok(())
            } else {
                println!("No builds available.");
                let error = io::Error::new(io::ErrorKind::Other, "No builds available.");
                Err(Box::new(error) as Box<dyn Error>)
            }
        })
    }
}