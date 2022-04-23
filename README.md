# Rust Game Ports

My ports of open source games to Rust, using different pure-Rust game engines.

## Ports plan

The completed ports are:

- Code the Classics Vol. 1/Boing: ported to [ggez](https://github.com/ggez/ggez)
- Code the Classics Vol. 1/Cavern: ported to [Macroquad](https://github.com/not-fl3/macroquad) (without using experimental features)

Work in progress:

- [Rusty Roguelike](https://github.com/thebracket/HandsOnRust) to [Bevy](https://github.com/bevyengine/bevy)

Planned:

- Other Code the Classics Vol. 1 games to [Fyrox](https://github.com/FyroxEngine/Fyrox)

## Source Projects

- Code the Classics Vol. 1 ([repository](https://github.com/Wireframe-Magazine/Code-the-Classics) and [book](https://wireframe.raspberrypi.org/books/code-the-classics1)): A very beautiful book on beginning game programming, written in Python; the project comprises of several games of different genres, each with a surprising complexity under the hood

## Projects structure/configuration

- all the projects share a Cargo configuration (in the repository root), with tweaks to speedup the compilation
- each project uses a `nightly` Rust toolchain, in order to take advantage of the Cargo configuration
- each project has a dedicated Visual Studio Code configuration (`.vscode` directory)
- the Rusty Roguelike project has one directory (workspace) for each step, but a shared `target` directory (in the parent directory of the projects)
