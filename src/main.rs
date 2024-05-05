use std::collections::HashMap;
use clap::{arg, command, Command};
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

    match args.get_one::<String>("game") {
        Some(game) => {
            if let Some(func) = games.get(&game.as_str()) {
                func.install_dependencies().await;
                func.install().await;
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
                    arg!(--"game" <PATH>)
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