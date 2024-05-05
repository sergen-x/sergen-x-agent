# sergen-x-agent

Rust CLI to manage to installation of game servers. Also
check out [sergen-x-contains](https://github.com/sergen-x/sergen-x-containers)
which provide this CLI alongside a docker base image.

## Installation

`.tar.gz` downloads for Linux are available from GitHub Releases.
Alternatively, it is also possible to clone this repository
and build using `rust 1.75+` by running `cargo build`.

## Support Games

- Minecraft (Vanilla)

## Usage

### Install
The installation command installs
the specified game and its required dependencies.

`sergen_x_agent install <game>`

### Start

The start command loads the game from the configuration and starts the server.

`sergen_x_agent start`

# TODO:

## Additional Games
- [ ] SteamCMD

## Add Minecraft server JARs
- [ ] [QuiltMC](https://quiltmc.org/en/)
- [ ] [Mohist/Banner](https://mohistmc.com/)
- [ ] [Purpur](https://purpurmc.org/)
- [ ] [Pufferfish](https://pufferfish.host/downloads)
- [ ] [ArcLight](https://github.com/IzzelAliz/Arclight)
- [ ] [Crucible](https://github.com/CrucibleMC/Crucible)
- [ ] [NeoForged](https://neoforged.net/news/2023-retrospection/)