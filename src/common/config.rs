use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub game: Game,
    pub dependencies: Vec<Dependency>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game: Game::default(),
            dependencies: Vec::new(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            name: String::new(),
            variant: None,
            version: String::new(),
            custom_startup_arguments: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub name: String, // eg Minecraft
    pub variant: Option<String>, // eg Vanilla
    pub version: String, // eg 1.20.4
    pub custom_startup_arguments: Option<String>, // eg modified Java flags
}

#[derive(Serialize, Deserialize)]
pub struct Dependency {
    pub name: String, // eg Java
    pub variant: Option<String>, // eg JRE
    pub version: String, // eg 21
    pub distribution: Option<String>, // eg OpenJDK
}

pub(crate) fn load(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if !path.exists() {
        // Todo: error handling if file doesn't exist
        return Ok(Config::default());
    }
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let cfg: Config = toml::from_str(&contents)?;
    Ok(cfg)
}

impl Game {
    pub fn is_valid(&self) -> bool {
        // Todo: Validate config
        true
    }
}

impl Dependency {
    pub fn is_valid(&self) -> bool {
        // Todo: Validate config
        true
    }
}

impl Config {
    pub fn is_valid(&self) -> bool {
        // Todo: Validate config
        true
    }

    pub fn set_game(&mut self, game: Game) {
        self.game = game;
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }

    pub fn get_dependency_by_name(&self, name: &str) -> Option<&Dependency> {
        self.dependencies.iter().find(|dep: &&Dependency| dep.name == name)
    }

    pub fn remove_dependency_by_name(&mut self, dependency_name: &str) {
        self.dependencies.retain(|dep: &Dependency| dep.name != dependency_name);
    }

    pub fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
        let toml_string = toml::to_string(self)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        let mut file = File::create(file_path)?;
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }
}