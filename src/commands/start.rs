use async_trait::async_trait;
use crate::{AsyncCommand};
use crate::common::config;
use crate::common::error::SergenError;

pub struct StartCommand;

#[async_trait]
impl AsyncCommand for StartCommand {
    async fn execute(
        &self,
        _args: &clap::ArgMatches
    ) ->  Result<(), SergenError> {
            match config::load("../app_config.toml") {
                Ok(config) => {
                    if let Some(_game) = config.game.variant {
                        println!("Game name not found.");
                        Ok(())
                    } else {
                        println!("Game not found.");
                        Ok(())
                    }
                }
                Err(err) => {
                    eprintln!("Failed to load config: {}", err);
                    Ok(())
                }
            }
    }
}