use crate::{AsyncCommand};
use crate::commands::command::SergenCommand;
use crate::common::config;

pub struct StartCommand;

impl AsyncCommand for StartCommand {
    fn execute(&self, _args: &clap::ArgMatches) ->  SergenCommand {
        Box::pin(async {
            match config::load("../app_config.toml") {
                Ok(config) => {
                    if let Some(game) = config.game.variant {
                        println!("Game name not found.");
                    } else {
                        println!("Game not found.");
                    }
                }
                Err(err) => {
                    eprintln!("Failed to load config: {}", err);
                }
            }
        })
    }
}