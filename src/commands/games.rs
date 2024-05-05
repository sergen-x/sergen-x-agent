use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::common::installer::Installer;
use crate::minecraft;

pub(crate) async fn get_games() -> Arc<Mutex<HashMap<String, Arc<Mutex<dyn Installer>>>>> {
    let games: Arc<Mutex<HashMap<String, Arc<Mutex<dyn Installer>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    {
        let mut games = games.lock().await;
        games.insert("minecraft".to_string(), Arc::new(Mutex::new(minecraft::main::Minecraft)));
    }
    games
}