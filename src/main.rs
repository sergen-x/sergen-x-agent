use std::collections::HashMap;
use clap::{Arg, command, Command};
use std::future::Future;
use std::ops::Deref;
use crate::common::installer::Installer;
use crate::commands::install::InstallCommand;
use crate::commands::start::StartCommand;
use crate::commands::command::AsyncCommand;

pub mod common;
pub mod dependencies;
pub mod minecraft;
pub mod commands;

async fn install_game(args: &clap::ArgMatches) {
    let mut games: HashMap<&str, Box<dyn Installer>> = HashMap::new();
    games.insert("minecraft", Box::new(minecraft::main::Minecraft));

    let version = args.get_one::<String>("version");
    let variant = args.get_one::<String>("variant");

    match args.get_one::<String>("game") {
        Some(game) => {
            if let Some(func) = games.get(&game.as_str()) {
                let _ = func.install_dependencies().await;
                let _ = func.install(version.cloned(), variant.cloned()).await;
            } else {
                println!("Game not recognized");
            }
        },
        None => {
            eprintln!("No game provided.");
        }
    }
}

#[tokio::main]
async fn main() {
    let mut commands: HashMap<&str, Box<dyn AsyncCommand>> = HashMap::new();
    commands.insert("install", Box::new(InstallCommand));
    commands.insert("start", Box::new(StartCommand));

    let matches = command!()
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
        .subcommand(
            Command::new("start")
                .about("Starts the game")
        )
        .get_matches();

    if let Some((name, args)) = matches.subcommand() {
        if let Some(command) = commands.get(name) {
            let future = command.execute(args);
            let handler = tokio::task::spawn(future);
            handler.await.expect("Task panicked");
        } else {
            println!("Command not recognized");
        }
    }
}