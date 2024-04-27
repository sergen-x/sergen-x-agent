use std::collections::HashMap;
use clap::{arg, command, Command};
use crate::common::installer::Installer;

pub mod common;
pub mod dependencies;
pub mod minecraft;

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
    let mut commands = HashMap::new();
    commands.insert("install", install_game);

    let matches = command!()
        .subcommand(
            Command::new("install")
                .about("Installs a game")
                .arg(
                    arg!(--"game" <PATH>)
                ),
        )
        .get_matches();

    if let Some((name, args)) = matches.subcommand() {
        if let Some(func) = commands.get(name) {
            func(args).await;
        } else {
            println!("Command not recognized");
        }
    }

    // let _ = minecraft::forge::versions::get_versions();
    // let _ = dependencies::java::adoptium::main::get_versions();
}