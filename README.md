# Rust Game Ports

My ports of open source games to Rust, using different pure-Rust game engines.

- [Rust Game Ports](#rust-game-ports)
  - [Summary](#summary)
  - [Games](#games)
    - [Boing/ggez](#boingggez)
    - [Cavern/Macroquad](#cavernmacroquad)
    - [Rusty Roguelike/Bevy ECS](#rusty-roguelikebevy-ecs)
    - [Soccer/Fyrox](#soccerfyrox)
  - [Source Projects/Libraries](#source-projectslibraries)
  - [Projects structure/configuration](#projects-structureconfiguration)

## Summary

The completed ports are:

| Source                               |      Game       |  Engine   |
| ------------------------------------ | :-------------: | :-------: |
| Code the Classics Vol. 1             |      Boing      |   ggez    |
| Code the Classics Vol. 1             |     Cavern      | Macroquad |
| Hands-on Rust: Effective Learning... | Rusty Roguelike |   Bevy    |
| Code the Classics Vol. 1             |     Soccer      |   Fyrox   |

I currently don't plan any further ports.

## Games

### Boing/ggez

A very straightforward port 🙂

### Cavern/Macroquad

Another very straightforward port 🙂

This port suffers from two Macroquad bugs:

- the music starts with a delay;
- on Nvidia cards, on Linux, CPU runs at 100% load (reported [here](https://github.com/not-fl3/macroquad/issues/275#issuecomment-939525290)).

Only the stable part of the library is used (the experimental [`scene`](https://github.com/not-fl3/macroquad/blob/master/src/experimental/scene.rs) API is not used).

### Rusty Roguelike/Bevy ECS

The ECS part of this game, originally Legion, has been ported to Bevy (the graphic/input library used is still [bracket-lib](https://github.com/amethyst/bracket-lib)).

I'm writing a mini book, ["Learn Bevy's ECS by ripping off someone else's project"](https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off), based on this project.

### Soccer/Fyrox

This port has designed with pseudo-immediate mode (to reflect the original logic); idiomatic Fyrox project should instead use scene graphs.

Widget APIs should also be used for the HUD/menus, but it seems that they don't support transparency, which is a requirement (feedback is pending on this topic).

The port suffers from two Fyrox bugs:

- at least one sound plays with a delay (reported [here](https://github.com/FyroxEngine/Fyrox/issues/324));
- some images render incorrectly (reported [here](https://github.com/FyroxEngine/Fyrox/issues/320)).

## Source Projects/Libraries

Projects:

- Code the Classics Vol. 1 ([repository](https://github.com/Wireframe-Magazine/Code-the-Classics) and [book](https://wireframe.raspberrypi.org/books/code-the-classics1)): A very beautiful book on beginning game programming, written in Python; the project comprises of several games of different genres, each with a surprising complexity under the hood
- Rust Roguelike/Hands-on Rust: Effective Learning... ([repository](https://github.com/thebracket/HandsOnRust) and [book](https://pragprog.com/titles/hwrust/hands-on-rust)): Another great book, on writing games in Rust; the project is divided in clear and progressive steps, and it's, in my opinion, the most effective way to learn working with ECSs (in this case, [Legion](https://github.com/amethyst/legion))

Libraries:

- [ggez](https://github.com/ggez/ggez)
- [Macroquad](https://github.com/not-fl3/macroquad)
- [Bevy](https://github.com/bevyengine/bevy)
- [Fyrox](https://github.com/FyroxEngine/Fyrox)

## Projects structure/configuration

- all the projects share a Cargo configuration (in the repository root), with tweaks to speedup the compilation
- each project uses a `nightly` Rust toolchain, in order to take advantage of the Cargo configuration
- each project has a dedicated Visual Studio Code configuration (`.vscode` directory)
- the Rusty Roguelike project has one directory (workspace) for each step, but a shared `target` directory (in the parent directory of the projects)
