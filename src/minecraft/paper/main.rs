use serde::Deserialize;
use tokio;
use crate::common::http;

#[derive(Debug, Deserialize)]
struct Projects {
    #[serde(rename="projects")]
    _projects: Vec<String>,
}

#[deride(Debug, Deserialize)]
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

#[deride(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Builds {
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "project_name")]
    pub project_name: String,
    pub version: String,
    pub builds: Vec<Build>,
}

#[deride(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Build {
    pub build: i64,
    pub time: String,
    pub channel: String,
    pub promoted: bool,
    pub changes: Vec<Change>,
    pub downloads: Downloads,
}

#[deride(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Change {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[deride(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub application: Application,
}

#[deride(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub name: String,
    pub sha256: String,
}

pub async fn get_projects() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.papermc.io/v2/projects";
    let projects: Projects = http::get(url).await?;
    Ok(())
}

pub async fn get_versions(
    project: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.papermc.io/v2/projects/{}}",
        project
    );
    let versions: Project = http::get(url).await?;
    Ok(())
}

pub async fn get_builds(
    project: &str,
    minecraft_version: &str, 
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.papermc.io/v2/projects/{}/versions/{}}/builds",
        project, minecraft_version,
    );
    let builds: Builds = http::get(url).await?;

    Ok(())
}

pub async fn download_build(
    project: &str, 
    minecraft_version: &str,
    build: &str,
    jar_name: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.papermc.io/v2/projects/{}}/versions/{}/builds/{}/downloads/{}}",
        project, minecraft_version, build, jar_name
    );
    http::download_file(&url).await?;
    Ok(())
}