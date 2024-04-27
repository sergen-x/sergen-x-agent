use std::collections::HashMap;
use clap::{arg, command, Command};

pub mod common;
pub mod dependencies;
pub mod minecraft;

async fn install_game(args: &clap::ArgMatches) {
    let mut games = HashMap::new();
    games.insert("minecraft", install_minecraft_vanilla_sever);

    match args.get_one::<String>("game") {
        Some(game) => {
            if let Some(func) = games.get(&game.as_str()) {
                func().await;
            } else {
                println!("Game not recognized");
            }
        },
        None => {
            eprintln!("No game provided.");
        }
    }
}

async fn install_minecraft_vanilla_sever() {
    let manifest = minecraft::vanilla::vanilla::get_all_versions().await.expect("Failed to fetch version manifest");
    let release_version = manifest.latest.release.clone();
    let version = manifest.get_download_url(&release_version)
        .await
        .expect("Failed to fetch download URL");

    if let Some(ref version_info) = version {
        let _ = version.expect("Missing version").download()
            .await
            .expect("Failed to download version");
    } else {

        panic!("VersionInfo is None, cannot download version");
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
    //let _ = dependencies::java::adoptium::main::download_version("21", "linux", "x64", "jdk").await;
}