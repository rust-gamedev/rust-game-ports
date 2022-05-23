use fyrox::{
    core::pool::Handle,
    dpi::PhysicalSize,
    engine::framework::prelude::*,
    engine::Engine,
    event::{ElementState, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    scene::{
        base::BaseBuilder,
        camera::{CameraBuilder, OrthographicProjection, Projection},
        dim2::rectangle::RectangleBuilder,
        node::Node,
        Scene,
    },
};

use crate::game::Game;
use crate::input_controller::InputController;
use crate::menu_state::MenuState;
use crate::resources::Resources;
use crate::state::State;
use crate::texture_node_builder::ImageNodesBuilder;
use crate::{controls::Controls, game};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 480.0;

pub struct GameGlobal {
    resources: Resources,
    scene: Handle<Scene>,
    camera: Handle<Node>,
    // Root of all the object nodes; on start, this is a phony node.
    background: Handle<Node>,
    input: InputController,
    game: Game,
    state: State,
    menu_state: Option<MenuState>,
    menu_num_players: u8,
    menu_difficulty: u8,
}

impl ImageNodesBuilder for GameGlobal {
    fn resources(&self) -> &Resources {
        &self.resources
    }
}

fn preset_window(engine: &Engine) {
    let window = engine.get_window();

    window.set_inner_size(PhysicalSize {
        width: WIDTH,
        height: HEIGHT,
    });

    window.set_resizable(false);
}

// Returns (scene, camera, (phony) backgroud node)
//
fn build_initial_scene(engine: &mut Engine) -> (Handle<Scene>, Handle<Node>, Handle<Node>) {
    let mut scene = Scene::new();

    let camera = CameraBuilder::new(BaseBuilder::new())
        .with_projection(Projection::Orthographic(OrthographicProjection {
            z_near: -0.1,
            z_far: 16.0,
            vertical_size: HEIGHT / 2.0,
        }))
        .build(&mut scene.graph);

    let background_node = RectangleBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

    let scene = engine.scenes.add(scene);

    (scene, camera, background_node)
}

impl GameState for GameGlobal {
    fn init(engine: &mut Engine) -> Self {
        preset_window(engine);

        let resources = Resources::load(&engine.resource_manager);

        let (scene, camera, background) = build_initial_scene(engine);

        let input = InputController::new();

        let game = Game::new(None, None, game::DEFAULT_DIFFICULTY);
        let state = State::Menu;
        let menu_state = Some(MenuState::NumPlayers);

        Self {
            resources,
            scene,
            camera,
            background,
            input,
            game,
            state,
            menu_state,
            menu_num_players: 1,
            menu_difficulty: 0,
        }
    }

    fn on_tick(&mut self, engine: &mut Engine, _dt: f32, _control_flow: &mut ControlFlow) {
        let scene = &mut engine.scenes[self.scene];

        // The simplest way to model a design that is as close a possible to a conventional 2d game
        // library, is to use the background as root node, and to dynamically add each sprite to draw
        // as node.
        //
        scene.graph.remove_node(self.background);

        self.update(engine);

        self.draw(engine);

        self.input.flush_event_received_state();
    }

    fn on_window_event(&mut self, _engine: &mut Engine, event: WindowEvent) {
        if let WindowEvent::KeyboardInput { input, .. } = event {
            if let Some(key_code) = input.virtual_keycode {
                use ElementState::*;

                match input.state {
                    Pressed => self.input.key_down(key_code),
                    Released => self.input.key_up(key_code),
                }
            }
        }
    }
}

// update() and game() don't have the Fyrox semantics, but they're added to make the comparison with
// the source code simpler.
//
impl GameGlobal {
    fn update(&mut self, _engine: &mut Engine) {
        use VirtualKeyCode::*;
        use {MenuState::*, State::*};

        match &self.state {
            Menu => {
                if self.input.is_key_just_pressed(Space) {
                    if let Some(NumPlayers) = self.menu_state {
                        // If we're doing a 2 player game, skip difficulty selection
                        if self.menu_num_players == 1 {
                            self.menu_state = Some(MenuState::Difficulty);
                        } else {
                            // Start 2P game
                            self.state = State::Play;
                            self.menu_state = None;
                            self.game = Game::new(
                                Some(Controls::new(0)),
                                Some(Controls::new(1)),
                                game::DEFAULT_DIFFICULTY,
                            )
                        }
                    } else {
                        // Start 1P game
                        self.state = State::Play;
                        self.menu_state = None;
                        self.game = Game::new(Some(Controls::new(0)), None, self.menu_difficulty);
                    }
                } else {
                    // Detect + act on up/down arrow keys
                    let mut selection_change: i8 = 0;
                    if self.input.is_key_just_pressed(Down) {
                        selection_change = 1
                    } else if self.input.is_key_just_pressed(Up) {
                        selection_change = -1;
                    }
                    if selection_change != 0 {
                        // sounds.move.play() // WRITEME
                        if let Some(MenuState::NumPlayers) = self.menu_state {
                            self.menu_num_players = if self.menu_num_players == 1 { 2 } else { 1 };
                        } else {
                            self.menu_difficulty =
                                (self.menu_difficulty as i8 + selection_change).rem_euclid(3) as u8
                        }
                    }
                }

                self.game.update()
            }
            _ => {}
        }
    }

    fn draw(&mut self, engine: &mut Engine) {
        // self.game.draw(); // WRITEME

        let scene = &mut engine.scenes[self.scene];

        use {MenuState::*, State::*};

        match &self.state {
            Menu => {
                let (image_i1, image_i2) = match self.menu_state.as_ref().unwrap() {
                    NumPlayers => (0, self.menu_num_players),
                    Difficulty => (1, self.menu_difficulty),
                };

                self.background =
                    self.build_image_node(&mut scene.graph, "menu", image_i1, image_i2);
            }
            Play => {
                //
            }
            GameOver => {
                //
            }
        }
    }
}
