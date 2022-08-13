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
  - [Port notes](#port-notes)
    - [Boing/ggez](#boingggez)
    - [Cavern/Macroquad](#cavernmacroquad)
    - [Rusty Roguelike/Bevy ECS](#rusty-roguelikebevy-ecs)
    - [Soccer/Fyrox](#soccerfyrox)
    - [Rusty Roguelike/Macroquad](#rusty-roguelikemacroquad)

## Summary

The completed ports are:

<!-- Add new releases at the bottom; this makes more likely to found outdated ports at the top -->

|                Game                |                                   Part of                                    | Source Language |               Source Libraries               |                   Port Libraries                   | Tested on |
| :--------------------------------: | :--------------------------------------------------------------------------: | :-------------: | :------------------------------------------: | :------------------------------------------------: | :-------: |
|           [Boing][Boing]           |             [Code the Classics Vol. 1][Code the Classics Vol. 1]             |     Python      |          [PyGame Zero][PyGame Zero]          |                  [ggez][ggez] 0.7                  |   Linux   |
|          [Cavern][Cavern]          |             [Code the Classics Vol. 1][Code the Classics Vol. 1]             |     Python      |          [PyGame Zero][PyGame Zero]          |             [Macroquad][Macroquad] 0.3             |   Linux   |
|          [Soccer][Soccer]          |             [Code the Classics Vol. 1][Code the Classics Vol. 1]             |     Python      |          [PyGame Zero][PyGame Zero]          |                [Fyrox][Fyrox] 0.26                 |   Linux   |
| [Rusty Roguelike][Rusty Roguelike] | [Hands-on Rust: Effective Learning...][Hands-on Rust: Effective Learning...] |      Rust       | [bracket-lib][bracket-lib], [Legion][Legion] | [bracket-lib][bracket-lib], [Bevy][Bevy] (ECS) 0.7 |   Linux   |
| [Rusty Roguelike][Rusty Roguelike] | [Hands-on Rust: Effective Learning...][Hands-on Rust: Effective Learning...] |      Rust       | [bracket-lib][bracket-lib], [Legion][Legion] |  [Macroquad][Macroquad] 0.3, [Legion][Legion] 0.3  |   Linux   |

<!-- Keep the entries of each group sorted by name -->

<!-- Game -->

[Boing]: https://github.com/Wireframe-Magazine/Code-the-Classics/tree/master/boing-master
[Cavern]: https://github.com/Wireframe-Magazine/Code-the-Classics/tree/master/cavern-master
[Rusty Roguelike]: https://github.com/thebracket/HandsOnRust
[Soccer]: https://github.com/Wireframe-Magazine/Code-the-Classics/tree/master/soccer-master

<!-- Part of... -->

[Code the Classics Vol. 1]: https://wireframe.raspberrypi.org/books/code-the-classics1
[Hands-on Rust: Effective Learning...]: https://pragprog.com/titles/hwrust/hands-on-rust

<!-- Source Libraries -->

[bracket-lib]: https://github.com/amethyst/bracket-lib
[Legion]: https://github.com/amethyst/legion
[PyGame Zero]: https://pygame-zero.readthedocs.io/en/stable

<!-- Port Libraries -->

[Bevy]: https://github.com/bevyengine/bevy
[Fyrox]: https://github.com/FyroxEngine/Fyrox
[ggez]: https://github.com/ggez/ggez
[Macroquad]: https://github.com/not-fl3/macroquad

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

Committed devs can perform ports that require a redesign (e.g. Fyrox Framework to Fyrox scripted).

It's not advised to perform a port that requires a language translation _and_ a redesign

### Low-level requirements

These are the low-level requirements for candidate projects (they will be automated, so they don't need to be actively take caren of 🙂):

- include a license in the port workspace (can be easily copied and edited from other similar projects)
- work on stable Rust (this is because they ubiquitous Rust Analyzer has some open issues with nighly Rust)
- be formatted according to `cargo fmt`
- be linted according to `cargo clippy -- -W clippy::correctness -D warnings`
- not have any unsafe code (it's not necessary for idiomatically ported games)
- not use any highly unidiomatic Rust (e.g. globals)

### High level guidelines

High level guidelines are under discussion. Generally speaking, ports should be performed according to the intended design of the game library used, since ports are meant to be examples for Rust game development. You'll be famous! 😎😂

## Screenshots

<!-- Keep the entries sorted by name -->

Boing:

![Boing](/images/readme/boing.png?raw=true)

Cavern:

![Cavern](/images/readme/cavern.png?raw=true)

Rusty Roguelike:

![Rusty Roguelike](/images/readme/rusty_roguelike.png?raw=true)

Soccer:

![Soccer](/images/readme/soccer.png?raw=true)

## Port notes

### Boing/ggez

A very straightforward port 🙂

This port suffers from one (Winit) bug:

- corruption when running on fullscreen (reported [here](https://github.com/ggez/ggez/issues/1066)).

### Cavern/Macroquad

Another very straightforward port 🙂

This port suffers from two Macroquad bugs:

- the music starts with a delay (reported by another developer [here](https://github.com/not-fl3/macroquad/issues/440));
- on Nvidia cards, on Linux, CPU runs at 100% load (reported [here](https://github.com/not-fl3/macroquad/issues/275#issuecomment-939525290)).

Only the stable part of the library is used (the experimental [`scene`](https://github.com/not-fl3/macroquad/blob/master/src/experimental/scene.rs) API is not used).

### Rusty Roguelike/Bevy ECS

The ECS part of this game, originally Legion, has been ported to Bevy (the graphic/input library used is still [bracket-lib](https://github.com/amethyst/bracket-lib)).

A mini book, ["Learn Bevy's ECS by ripping off someone else's project"](https://saveriomiroddi.github.io/learn_bevy_ecs_by_ripping_off), is based on this project.

### Soccer/Fyrox

This port required a redesign, in order to move to a scene graph.

The port suffers from one Fyrox bug:

- at least one sound plays with a delay (reported [here](https://github.com/FyroxEngine/Fyrox/issues/324)).

### Rusty Roguelike/Macroquad

The graphics portion of the Rusty Roguelike has been ported to Macroquad. The ECS used is still Legion, and the pathfinding is from [bracket-lib](https://github.com/amethyst/bracket-lib)).
