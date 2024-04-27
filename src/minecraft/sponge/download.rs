use serde::Deserialize;
use crate::common::http;

#[derive(Debug, Deserialize)]
struct Github {
    owner: String,
    repo: String,
}

#[derive(Debug, Deserialize)]
struct LatestVersion {
    version: String,
    dependencies: Dependencies,
}

#[derive(Debug, Deserialize)]
struct Dependencies {
    forge: String,
    minecraft: String,
    spongeapi: String,
    #[serde(default)]
    sponge: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BuildTypes {
    bleeding: Bleeding,
}

#[derive(Debug, Deserialize)]
struct Bleeding {
    latest: LatestVersion,
}

#[derive(Debug, Deserialize)]
struct Versions {
    stable: LatestVersion,
    recommended: Recommended,
}

#[derive(Debug, Deserialize)]
struct Recommended {
    version: String,
    dependencies: Dependencies,
}

#[derive(Debug, Deserialize)]
struct VersionResponse {
    name: String,
    #[serde(rename="pluginId")]
    plugin_id: String,
    github: Github,
    #[serde(rename="buildTypes")]
    build_types: BuildTypes,
    dependencies: Dependencies,
}

pub fn list_projects() -> Vec<&'static str> {
    let types = vec!["spongevanilla", "spongeforge"];
    types
}

pub async fn get_versions(
    project: &str,
) -> Result<VersionResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://dl-api.spongepowered.org/v1/org.spongepowered/{}",
        project
    );
    let versions: VersionResponse = http::get(&url).await?;
    Ok(versions)
}

impl VersionResponse {
    pub async fn download(
        &self,
        version: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://repo.spongepowered.org/maven/org/spongepowered/{}/{}/{}.jar",
            self.name, version, version
        );
        http::download_file(&url).await?;
        Ok(())
    }
}