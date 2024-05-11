use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{AsyncCommand};
use crate::commands::command::SergenCommand;
use crate::common::installer::Installer;
use crate::commands::games::get_games;

pub struct InstallCommand;

impl AsyncCommand for InstallCommand {
    fn execute(&self, args: &clap::ArgMatches) ->  SergenCommand {
        let args_clone = args.clone();

        Box::pin(async move {
            let version = args_clone.get_one::<String>("version");
            let variant = args_clone.get_one::<String>("variant");
            let games: Arc<Mutex<HashMap<String, Arc<Mutex<dyn Installer>>>>> = get_games().await;
            match args_clone.get_one::<String>("game") {
                Some(game) => {
                    if let Some(game_arc_mutex) = games.lock().await.get(game) {
                        let game = Arc::clone(&game_arc_mutex);
                        let game = game.lock().await;
                        // Todo: error handling
                        let _ = game.install_dependencies().await;
                        let _ = game.install(version.cloned(), variant.cloned()).await;
                    } else {
                        println!("Game not recognized");
                    }
                }
                None => {
                    eprintln!("No game provided.");
                }
            }
        })
    }
}