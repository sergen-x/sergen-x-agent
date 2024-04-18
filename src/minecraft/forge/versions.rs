use serde::Deserialize;
use crate::common::http;
use crate::common::kv::pair::Map;

#[derive(Debug, Deserialize)]
struct ForgeVersions {
    #[serde(rename="homepage")]
    _homepage: String,
    promos: Map<String, String>,
}

pub async fn get_versions() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
    let versions: ForgeVersions = http::get(url).await?;
    println!("{:?}", versions._homepage);
    println!("{:?}", versions.promos);

    Ok(())
}
