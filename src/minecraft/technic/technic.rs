use serde::{Deserialize, Serialize};
use crate::common::installer::InstallerFuture;
use crate::common::http;

#[derive(Serialize, Deserialize)]
struct Modpack {
    pub id: i64,
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub user: String,
    pub url: Option<String>,
    #[serde(rename = "platformUrl")]
    pub platform_url: String,
    pub minecraft: String,
    pub ratings: i64,
    pub installs: i64,
    pub runs: i64,
    pub description: String,
    pub tags: Option<String>,
    #[serde(rename = "isServer")]
    pub is_server: bool,
    #[serde(rename = "isOfficial")]
    pub is_official: bool,
    pub version: String,
    #[serde(rename = "forceDir")]
    pub force_dir: bool,
    pub feed: Vec<Updates>,
    pub icon: Info,
    pub logo: Info,
    pub background: Info,
    pub solder: String,
    #[serde(rename = "discordServerId")]
    pub discord_server_id: String,
    #[serde(rename = "serverPackUrl")]
    pub server_pack_url: String,
}

#[derive(Serialize, Deserialize)]
struct Updates {
    pub user: String,
    pub date: i64,
    pub content: String,
    pub avatar: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
struct Info {
    pub url: String,
    pub md5: String,
}

fn install(modpack_name: &str) -> InstallerFuture {
    // Todo: We are supposed to pass the build ID from
    // https://api.technicpack.net/launcher/version/stable4
    // But it works perfectly fine passing a dummy value, for now.
    let url = format!(
        "https://api.technicpack.net/modpack/{modpack_name}?build=latest"
    );
    Box::pin(async move {
        let modpack: Modpack = http::get(&url).await?;
        http::download_file(&modpack.server_pack_url);
        Ok(())
    })
}
