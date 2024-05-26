use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::commands::command::AsyncCommand;
use crate::commands::install::InstallCommand;
use crate::commands::start::StartCommand;
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

pub(crate) async fn get_commands() -> Arc<Mutex<HashMap<String, Arc<Mutex<dyn AsyncCommand>>>>> {
    let commands: Arc<Mutex<HashMap<String, Arc<Mutex<dyn AsyncCommand>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    {
        let mut commandmap = commands.lock().await;

        commandmap.insert("install".to_string(), Arc::new(Mutex::new(InstallCommand)));
        commandmap.insert("start".to_string(), Arc::new(Mutex::new(StartCommand)));

    }
    commands
}