# Minecraft  Variants

This directory contains implementations for different types of Minecraft servers.
Each implementation serves a specific purpose and caters to different needs within 
the Minecraft community.

## Server Software

### Vanilla
- **Description**: The official, unmodified server software from Mojang.
- **Usage**: Ideal for server owners wanting a vanilla game experience, without modifications.

| Implementation | Description               | 
|----------------|---------------------------|
| Mojang         | The official, base server |

### Plugins
- **Description**: Plugin servers are based on platforms like Spigot or Paper,
which allow server owners to add functionality through plugins without having to modify
the game's source code.
- **Usage**: Ideal for server owners who want to customize their server experience
without extensive programming or modding.
  
| Implementation                                  | Description                  | 
|-------------------------------------------------|------------------------------|
| [Paper](https://papermc.io/) (Recommended)      | A performant fork of Spigot  |
| [Purpur](https://purpurmc.org/)                 | Offers gameplay enhancements |
| [Pufferfish](https://pufferfish.host/downloads) | An enterprise fork of Paper  |
| [SpongeVanilla](https://spongepowered.org/)     | For sponge                   |

### Modded
- **Description**: Modded servers involve modifying the game's source code using 
tools like Forge or Fabric to introduce new features, mechanics, or content.
This requires mods to be installed both on the client (person joining the server)
and server.
- **Usage**: Suited for players seeking a significantly altered gameplay experience with custom mods, new dimensions, or unique mechanics.

| Implementation                                                  | Description                 | 
|-----------------------------------------------------------------|-----------------------------|
| [Quilt](https://quiltmc.org/) (Recommended for Fabric mods)     | A performant fork of Fabric |
| [Fabric](https://fabricmc.net/)                                 | The base modding API        |
| [NeoForge](https://neoforged.net/) (Recommended for Forge mods) | A performant fork of Forge  |
| [Forge](https://docs.minecraftforge.net/en/latest/)             | The base modding API        |

### Hybrid
- **Description**: Hybrid servers are a frankenstein blend of plugin
and modded implementations, combining the flexibility of plugins with
the extensive customization options of mods.
- **Usage**: Suitable for server owners who want to offer both vanilla-like gameplay
with additional features provided by both mods and plugins.
These implementations are NOT RECOMMENDED, and it is suggested to use
either plugins OR mods where possible, not in combination.
  
| Implementation                                 | Description            |
|------------------------------------------------|------------------------|
| [SpongeForge](https://spongepowered.org/)      | A forge/sponge hybrid  |
| [Mohist](https://mohistmc.com/software/mohist) | A forge/spigot hybrid  |
| [Banner](https://mohistmc.com/software/banner) | A fabric/spigot hybrid |