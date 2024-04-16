use serde::Deserialize;
use std::collections::HashMap;
use crate::common::http;
use tokio;

#[derive(Debug, Deserialize)]
struct ForgeVersions {
    #[serde(rename="homepage")]
    _homepage: String,
    promos: HashMap<String, String>,
}

#[tokio::main]
pub async fn get_versions() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
    let versions: ForgeVersions = http::get(url).await?;
    
    println!("{:?}", versions.promos);

    Ok(())
}
