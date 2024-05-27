use crate::common::config;
use crate::common::error::SergenError;
use crate::AsyncCommand;
use async_trait::async_trait;

pub struct StartCommand;

#[async_trait]
impl AsyncCommand for StartCommand {
    async fn execute(
        &self,
        _args: &clap::ArgMatches,
    ) -> Result<(), SergenError> {
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
