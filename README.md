# Rust Game Ports

My ports of open source games to Rust, using different pure-Rust game engines.

## Ports plan

The completed ports are:

- Code the Classics Vol. 1/Boing: ported to [ggez](https://github.com/ggez/ggez)
- Code the Classics Vol. 1/Cavern: ported to [Macroquad](https://github.com/not-fl3/macroquad) (without using experimental features)
- Hands-on Rust: Effective Learning.../Rusty Roguelike: ported to [Bevy](https://github.com/bevyengine/bevy)

Work in progress:

- Code the Classics Vol. 1/TBD: porting to [Fyrox](https://github.com/FyroxEngine/Fyrox)

## Source Projects

- Code the Classics Vol. 1 ([repository](https://github.com/Wireframe-Magazine/Code-the-Classics) and [book](https://wireframe.raspberrypi.org/books/code-the-classics1)): A very beautiful book on beginning game programming, written in Python; the project comprises of several games of different genres, each with a surprising complexity under the hood
- Rust Roguelike/Hands-on Rust: Effective Learning... ([repository](https://github.com/thebracket/HandsOnRust) and [book](https://pragprog.com/titles/hwrust/hands-on-rust)): Another great book, on writing games in Rust; the project is divided in clear and progressive steps, and it's, in my opinion, the most effective way to learn working with ECSs (in this case, [Legion](https://github.com/amethyst/legion))

## Projects structure/configuration

- all the projects share a Cargo configuration (in the repository root), with tweaks to speedup the compilation
- each project uses a `nightly` Rust toolchain, in order to take advantage of the Cargo configuration
- each project has a dedicated Visual Studio Code configuration (`.vscode` directory)
- the Rusty Roguelike project has one directory (workspace) for each step, but a shared `target` directory (in the parent directory of the projects)
