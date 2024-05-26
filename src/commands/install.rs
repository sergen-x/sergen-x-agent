use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::{AsyncCommand};
use crate::common::installer::Installer;
use crate::commands::games::get_games;
use crate::common::error::SergenError;

pub struct InstallCommand;

#[async_trait]
impl AsyncCommand for InstallCommand {
    async fn execute(
        &self,
        args: &clap::ArgMatches
    ) ->  Result<(), SergenError> {
        let args_clone = args.clone();

            let version = args_clone.get_one::<String>("version");
            let variant = args_clone.get_one::<String>("variant");
            let games: Arc<Mutex<HashMap<String, Arc<Mutex<dyn Installer>>>>> = get_games().await;
            match args_clone.get_one::<String>("game") {
                Some(game) => {
                    if let Some(game_arc_mutex) = games.lock().await.get(game) {
                        let game = Arc::clone(&game_arc_mutex);
                        let game = game.lock().await;
                        // Todo: error handling
                        game.install_dependencies().await;
                        game.install(version.cloned(), variant.cloned()).await;
                        Ok(())
                    } else {
                        println!("Game not recognized");
                        Ok(())
                    }
                }
                None => {
                    eprintln!("No game provided.");
                    Ok(())
                }
            }
    }
}