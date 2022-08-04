# Rust Game Ports

Official host of games ported using Rust game libraries.

- [Rust Game Ports](#rust-game-ports)
  - [Summary](#summary)
  - [Screenshots](#screenshots)
  - [Ports](#ports)
    - [Boing/ggez](#boingggez)
    - [Cavern/Macroquad](#cavernmacroquad)
    - [Rusty Roguelike/Bevy ECS](#rusty-roguelikebevy-ecs)
    - [Soccer/Fyrox](#soccerfyrox)
  - [Source Projects](#source-projects)
  - [Game Libraries](#game-libraries)

## Summary

The completed ports are:

|      Game       |                Source                | Source Language/Libraries |     Port Libraries      | Tested on |
| :-------------: | :----------------------------------: | :-----------------------: | :---------------------: | :-------: |
|      Boing      |       Code the Classics Vol. 1       |    Python, PyGame Zero    |          ggez           |   Linux   |
|     Cavern      |       Code the Classics Vol. 1       |    Python, PyGame Zero    |        Macroquad        |   Linux   |
| Rusty Roguelike | Hands-on Rust: Effective Learning... |    bracket-lib, Legion    | bracket-lib, Bevy (ECS) |   Linux   |
|     Soccer      |       Code the Classics Vol. 1       |    Python, PyGame Zero    |          Fyrox          |   Linux   |

## Screenshots

Boing:

![Boing](/images/readme/boing.png?raw=true)

Cavern:

![Cavern](/images/readme/cavern.png?raw=true)

Rusty Roguelike:

![Rusty Roguelike](/images/readme/rusty_roguelike.png?raw=true)

Soccer:

![Soccer](/images/readme/soccer.png?raw=true)

## Ports

### Boing/ggez

A very straightforward port 🙂

This port suffers from one bug:

- corruption when running on fullscreen (reported [here](https://github.com/ggez/ggez/issues/1066)).

### Cavern/Macroquad

Another very straightforward port 🙂

This port suffers from two Macroquad bugs:

- the music starts with a delay (reported by another developer [here](https://github.com/not-fl3/macroquad/issues/440));
- on Nvidia cards, on Linux, CPU runs at 100% load (reported [here](https://github.com/not-fl3/macroquad/issues/275#issuecomment-939525290)).

Only the stable part of the library is used (the experimental [`scene`](https://github.com/not-fl3/macroquad/blob/master/src/experimental/scene.rs) API is not used).

### Rusty Roguelike/Bevy ECS

The ECS part of this game, originally Legion, has been ported to Bevy (the graphic/input library used is still [bracket-lib](https://github.com/amethyst/bracket-lib)).

I wrote a mini book, ["Learn Bevy's ECS by ripping off someone else's project"](https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off), based on this project.

### Soccer/Fyrox

This port required a non-trivial restructuring, in order to move to a scene-graph based design.

The port suffers from two Fyrox bugs:

- at least one sound plays with a delay (reported [here](https://github.com/FyroxEngine/Fyrox/issues/324));
- some images render incorrectly (reported [here](https://github.com/FyroxEngine/Fyrox/issues/320)).

## Source Projects

- Code the Classics Vol. 1 ([repository](https://github.com/Wireframe-Magazine/Code-the-Classics) and [book](https://wireframe.raspberrypi.org/books/code-the-classics1))
- Rust Roguelike/Hands-on Rust: Effective Learning... ([repository](https://github.com/thebracket/HandsOnRust) and [book](https://pragprog.com/titles/hwrust/hands-on-rust))

## Game Libraries

- [ggez](https://github.com/ggez/ggez)
- [Macroquad](https://github.com/not-fl3/macroquad)
- [Bevy](https://github.com/bevyengine/bevy)
- [Fyrox](https://github.com/FyroxEngine/Fyrox)
