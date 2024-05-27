use crate::commands::command::AsyncCommand;
use crate::commands::games::get_commands;

use clap::{command, Arg, ArgMatches, Command};
use std::collections::HashMap;


use std::sync::Arc;

pub mod commands;
pub mod common;
pub mod dependencies;
pub mod minecraft;

#[tokio::main]
async fn main() {
    let matches: Arc<ArgMatches> = Arc::new(
        command!()
            .subcommand(
                Command::new("install")
                    .about("Installs a game")
                    .arg(
                        Arg::new("game")
                            .index(1)
                            .required(true)
                            .help("Name of the game to install"),
                    )
                    .arg(
                        Arg::new("variant")
                            .index(2)
                            .required(false)
                            .help("The variant of the game to install"),
                    )
                    .arg(
                        Arg::new("version")
                            .index(3)
                            .required(false)
                            .help("The version of the game to install"),
                    ),
            )
            .subcommand(Command::new("start").about("Starts the game"))
            .get_matches(),
    );

    let commands: Arc<
        tokio::sync::Mutex<
            HashMap<String, Arc<tokio::sync::Mutex<dyn AsyncCommand>>>,
        >,
    > = get_commands().await;

    let subcommand = matches
        .subcommand()
        .map(|(name, args)| (name.to_string(), args.clone()));

    if let Some((name, args)) = subcommand {
        let cloned_commands = Arc::clone(&commands);
        let handler = tokio::task::spawn(async move {
            let command = {
                let locked_commands = cloned_commands.lock().await;
                locked_commands.get(&name).cloned()
            };

            if let Some(command) = command {
                let cmd = command.lock().await;
                cmd.execute(&args).await
            } else {
                println!("Command not recognized");
                Ok(())
            }
        });
        handler.await.expect("Task panicked");
    }
}
