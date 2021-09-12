```mermaid
classDiagram

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Game {
  bunner
  eagle
  frame
  rows
  scroll_pos
  update()
  draw()
  score()
  update()$
  draw()$
}

Bunner <-- Game : update -> \n Bunner.update
Eagle <-- Game : update -> \n Eagle.update
Row <-- Game : update -> \n Row.update

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class State {
  <<enum>>
  MENU
  PLAY
  GAME_OVER
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% draw() children, sending offset
%% update() children
%%
class MyActor {
  x
  y
  children
  draw()
  update()
}

Actor <|-- MyActor

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% next(): overridden
%% collide(): return the colliding child, if any
%% push(): overridden; default=0
%% check_collision(): overridden; default=player alive
%% allow_movement(): prevent the player from exiting
%%
class Row {
  index
  next()
  collide()
  push()
  check_collision()
  allow_movement()
}

MyActor <|-- Row

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% dx: direction
%% init(): populate with random children (of child_type)
%% update(): manage dropping and creating children
%%
class ActiveRow {
  child_type
  timer
  dx
}

Row <|-- ActiveRow

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Grass {
  hedge_row_index
  hedge_mask
  allow_movement()
  next()
}

Row <|-- Grass

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Dirt {
  next()
}

Row <|-- Dirt

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Pavement {
  next()
}

Row <|-- Pavement

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Rail {
  next()
  update()
  check_collision()
  next()
}

Row <|-- Rail

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Water {
  update()
  push()
  check_collision()
  next()
}

ActiveRow <|-- Water

Game <-- Water : update -> \n Game.bunner

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Road {
  update()
  push()
  check_collision()
  next()
}

ActiveRow <|-- Road

Game <-- Road : update -> \n Game.bunner

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%% The static methods are actually global
%%
class Hedge {
  generate_hedge_mask()$
  classify_hedge_segment()$
}

MyActor <|-- Hedge

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% update(): increase y
%%
class Eagle {
  update()
}

MyActor <|-- Eagle

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class PlayerState {
  <<enum>>
  ALIVE
  SPLAT
  SPLASH
  EAGLE
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

class Bunner {
  state
  direction
  timer
  input_queue
  min_y

  handle_input()
  update()
}

MyActor <|-- Bunner

Game <-- Bunner : handle_input -> \n Row.allow_movement
Game <-- Bunner : update       -> \n Row.check_collision \n Row.push \n Row.children.insert(splat)

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% update(): updates x
%%
class Mover {
  dx
  update()
}

MyActor <|-- Mover

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% only handles sounds
%%
class Car

Mover <|-- Car

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% only handles image
%%
class Log

Mover <|-- Log

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%% only handles image
%%
class Train

Mover <|-- Train
