# Rust Game Ports

Official host of games ported using Rust game libraries.

This project is intended to inform and help developers in the area of *actual* game programming with Rust and Rust game engines.

If you're a developer planning a contribution, it's **crucial** to read the [Contribution Infos](#contribution-infos) section.

- [Rust Game Ports](#rust-game-ports)
  - [Summary](#summary)
  - [Contribution Infos](#contribution-infos)
    - [Choosing and implementing a port](#choosing-and-implementing-a-port)
    - [Low-level requirements](#low-level-requirements)
    - [High level guidelines](#high-level-guidelines)
  - [Screenshots](#screenshots)
  - [Ports](#ports)
    - [Boing/ggez](#boingggez)
    - [Cavern/Macroquad](#cavernmacroquad)
    - [Rusty Roguelike/Bevy ECS](#rusty-roguelikebevy-ecs)
    - [Soccer/Fyrox](#soccerfyrox)
    - [Rusty Roguelike/Macroquad](#rusty-roguelikemacroquad)
  - [Source Projects](#source-projects)
  - [Libraries](#libraries)
    - [Sources](#sources)
    - [Ports](#ports-1)

## Summary

The completed ports are:

|      Game       |                Source                | Source Language/Libraries |       Port Libraries        | Tested on |
| :-------------: | :----------------------------------: | :-----------------------: | :-------------------------: | :-------: |
|      Boing      |       Code the Classics Vol. 1       |    Python, PyGame Zero    |          ggez 0.7           |   Linux   |
|     Cavern      |       Code the Classics Vol. 1       |    Python, PyGame Zero    |        Macroquad 0.3        |   Linux   |
| Rusty Roguelike | Hands-on Rust: Effective Learning... |    bracket-lib, Legion    | bracket-lib, Bevy (ECS) 0.7 |   Linux   |
| Rusty Roguelike | Hands-on Rust: Effective Learning... |    bracket-lib, Legion    |    Macroquad 0.3, Legion    |   Linux   |
|     Soccer      |       Code the Classics Vol. 1       |    Python, PyGame Zero    |         Fyrox 0.26          |   Linux   |

## Contribution Infos

Contributions are welcome!

Since this project is intended to be educational above all, it's important for the candidate ports to constitute proper examples 😄

In this section you'll find some suggestions to start a port, and the low and high level guidelines. Feel free to skip the first, but don't underestimate it! 😉

### Choosing and implementing a port

Devs motivated to implement a port are advised to be very careful with what they choose 😄

While some ports are straightforward, for example Rust+immediate mode library ("IML") -> Rust+IML, other types of port can be subtly challenging:

- Dynamic language+IML (e.g. Python+Pygame -> Rust+ggez) are challenging for many reasons:
  - It's difficult to understand the exact structure/state of the classes/instances
  - Globals may be used
  - Numeric types can be difficult to ascertain, and the descriptions may be buggy (e.g. a number may accept negative values, while being described as accepting only positive ones)
- IML -> Retained mode library are very time consuming, as they require a full redesign, an example can be porting a Rust game from ggez to Fyrox
- Object oriented -> ECS is another time consuming one, since it requires a full redesign

For beginners, a good starting point is to take a Rust+IML project, e.g. Boing, and convert it to another IML, e.g. Macroquad.

More adventurous devs can start from a Python/PyGame source, and port it to Rust+IML.

Committed devs can perform ports that require a redesign (e.g. Fyrox Framework to Fyrox Scripted).

It's not advised to perform a port that requires a language translation _and_ a redesign

### Low-level requirements

These are the low-level requirements for candidate projects (they will be automated, so they don't need to be actively take caren of 🙂):

- include a license in the root workspace (can be easily copied and edited from other similar projects)
- work on stable Rust (this is because they ubiquitous Rust Analyzer has some open issues with nighly Rust)
- be formatted according to `cargo fmt`
- be linted according to `cargo clippy -- -W clippy::correctness -D warnings`
- not have any unsafe code (it's not necessary for games)
- not use any highly unidiomatic Rust (e.g. globals)
- use symlinks for the resource directories, if they're shared with the source project
- add the source code, if not present already

### High level guidelines

High level guidelines are under discussion. Generally speaking, ports should be performed idiomatically (with particular regard to the game library used), since ports are meant to be examples for Rust game development. You'll be famous! 😎😂

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

### Rusty Roguelike/Macroquad

The graphics portion of the Rusty Roguelike has been ported to Macroquad. The ECS used is still Legion, and the pathfinding is from [bracket-lib](https://github.com/amethyst/bracket-lib)).

## Source Projects

- Code the Classics Vol. 1 ([repository](https://github.com/Wireframe-Magazine/Code-the-Classics) and [book](https://wireframe.raspberrypi.org/books/code-the-classics1))
- Rust Roguelike/Hands-on Rust: Effective Learning... ([repository](https://github.com/thebracket/HandsOnRust) and [book](https://pragprog.com/titles/hwrust/hands-on-rust))

## Libraries

### Sources

- [PyGame Zero](https://pygame-zero.readthedocs.io/en/stable)
- [bracket-lib](https://github.com/amethyst/bracket-lib)

### Ports

- [ggez](https://github.com/ggez/ggez)
- [Macroquad](https://github.com/not-fl3/macroquad)
- [Bevy](https://github.com/bevyengine/bevy)
- [Fyrox](https://github.com/FyroxEngine/Fyrox)
