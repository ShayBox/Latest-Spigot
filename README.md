<div align="center">
  <a href="https://discord.shaybox.com">
    <img alt="Discord" src="https://img.shields.io/discord/824865729445888041?color=404eed&label=Discord&logo=Discord&logoColor=FFFFFF">
  </a>
  <a href="https://github.com/shaybox/latestspigot/releases/latest">
    <img alt="Downloads" src="https://img.shields.io/github/downloads/shaybox/latestspigot/total?color=3fb950&label=Downloads&logo=github&logoColor=FFFFFF">
  </a>
</div>

# LatestSpigot

Rust CLI wrapper for the Spigot BuildTool

### Requirements:
| Minecraft | Recommended |
|-----------|-------------|
| < 1.17    | [Java 8]    |
| 1.17      | [Java 16]   |
| 1.17 >    | [Java 17]   |

## Usage:
```
Option                                 Description
------                                 -----------
--compile <[NONE,CRAFTBUKKIT,SPIGOT]>  Software to compile
--compile-if-changed                   Run BuildTools only when changes are
                                         detected in the repository
--dev                                  Development mode
--disable-certificate-check            Disable HTTPS certificate check
--disable-java-check                   Disable Java version check
--dont-update                          Don't pull updates from Git
--generate-docs                        Generate Javadoc jar
--generate-source                      Generate source jar
--help                                 Show the help
-o, --output-dir <File>                Final jar output directory (default: .)
--remapped                             Produce and install extra remapped jars
--rev <String>                         Version to build (default: latest)
--skip-compile                         Skip compilation
```

[Download](https://github.com/ShayBox/LatestSpigot/releases/latest)

[Java 8]: https://adoptium.net/temurin/releases?version=8
[Java 16]: https://adoptium.net/temurin/releases?version=16
[Java 17]:https://adoptium.net/temurin/releases?version=17
