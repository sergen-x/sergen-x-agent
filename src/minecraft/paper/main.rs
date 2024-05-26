use serde::Deserialize;
use serde_json;
use crate::common::error::SergenError;
use crate::common::http;

#[derive(Debug, Deserialize)]
struct Projects {
    #[serde(rename="projects")]
    _projects: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Project {
    #[serde(rename="project_id")]
    _project_id: String,
    #[serde(rename="project_name")]
    _project_name: String,
    #[serde(rename="version_groups")]
    _version_groups: Vec<String>,
    #[serde(rename="versions")]
    _versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Builds {
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "project_name")]
    pub project_name: String,
    pub version: String,
    pub builds: Vec<Build>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub build: i64,
    pub time: String,
    pub channel: String,
    pub promoted: bool,
    pub changes: Vec<Change>,
    pub downloads: Downloads,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub application: Application,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub name: String,
    pub sha256: String,
}

pub async fn get_projects() -> Result<(), SergenError> {
    let url = "https://api.papermc.io/v2/projects";
    let projects: Projects = http::get(url).await?;
    Ok(())
}

pub async fn get_versions(
    project: &str
) -> Result<(), SergenError> {
    let url = format!(
        "https://api.papermc.io/v2/projects/{}",
        project
    );
    let versions: Project = http::get(&url).await?;
    Ok(())
}

pub async fn get_builds(
    project: &str,
    minecraft_version: &str, 
) -> Result<(), SergenError> {
    let url = format!(
        "https://api.papermc.io/v2/projects/{}/versions/{}/builds",
        project, minecraft_version,
    );
    let builds: Builds = http::get(&url).await?;

    Ok(())
}

pub async fn download_build(
    project: &str, 
    minecraft_version: &str,
    build: &str,
    jar_name: &str
) -> Result<(), SergenError> {
    let url = format!(
        "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}/downloads/{}",
        project, minecraft_version, build, jar_name
    );
    http::download_file(&url).await?;
    Ok(())
}