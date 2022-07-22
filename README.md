# Rust Game Ports

My ports of open source games to Rust, using different pure-Rust game engines.

- [Rust Game Ports](#rust-game-ports)
  - [Summary](#summary)
  - [Notes](#notes)
  - [Games](#games)
    - [Boing/ggez](#boingggez)
    - [Cavern/Macroquad](#cavernmacroquad)
    - [Rusty Roguelike/Bevy ECS](#rusty-roguelikebevy-ecs)
    - [Soccer/Fyrox](#soccerfyrox)
  - [Source Projects/Libraries](#source-projectslibraries)
  - [Projects structure/configuration](#projects-structureconfiguration)

## Summary

The completed ports are:

| Source                               |      Game       |   Engine   |
| ------------------------------------ | :-------------: | :--------: |
| Code the Classics Vol. 1             |      Boing      |    ggez    |
| Code the Classics Vol. 1             |     Cavern      | Macroquad  |
| Hands-on Rust: Effective Learning... | Rusty Roguelike | Bevy (ECS) |
| Code the Classics Vol. 1             |     Soccer      |   Fyrox    |

I currently don't plan any further ports.

## Notes

Nightly Rust may be required for some games. The games have been developed tested on Linux; if anybody finds issues with Windows, open an issue and I'll quickly look into it 😄

The games have been carefully ported; some design details have been implemented non-idiomatically; this has been intentional, in order not to diverge too much from the original projects (and therefore, to make direct comparison not too hard). Nonetheless, if you have doubts/suggestions about the quality of the code, you're invited to open an issue 😄

## Games

### Boing/ggez

![Boing](/images/readme/boing.png?raw=true)

A very straightforward port 🙂

This port suffers from one bug:

- corruption when running on fullscreen (reported [here](https://github.com/ggez/ggez/issues/1066)).

### Cavern/Macroquad

![Cavern](/images/readme/cavern.png?raw=true)

Another very straightforward port 🙂

This port suffers from two Macroquad bugs:

- the music starts with a delay;
- on Nvidia cards, on Linux, CPU runs at 100% load (reported [here](https://github.com/not-fl3/macroquad/issues/275#issuecomment-939525290)).

Only the stable part of the library is used (the experimental [`scene`](https://github.com/not-fl3/macroquad/blob/master/src/experimental/scene.rs) API is not used).

### Rusty Roguelike/Bevy ECS

![Rusty Roguelike](/images/readme/rusty_roguelike.png?raw=true)

The ECS part of this game, originally Legion, has been ported to Bevy (the graphic/input library used is still [bracket-lib](https://github.com/amethyst/bracket-lib)).

I wrote a mini book, ["Learn Bevy's ECS by ripping off someone else's project"](https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off), based on this project.

Please note that in the steps from 10.x to 15.01 (that is, except the last), the FOV flickers. The fix (see [fix commit](/../../commit/71655f2d7e)) can be easily backported to the previous steps; if anybody wants to contribute the backport, they're very welcome 😄.

### Soccer/Fyrox

![Soccer](/images/readme/soccer.png?raw=true)

This port required a non-trivial restructuring, in order to move to a scene-graph based design.

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

- each project has a dedicated Visual Studio Code configuration (`.vscode` directory)
- the Rusty Roguelike project has one directory (workspace) for each step, but a shared `target` directory (in the parent directory of the projects)
