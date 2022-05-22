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

use crate::input_controller::InputController;
use crate::menu_state::MenuState;
use crate::resources::Resources;
use crate::state::State;
use crate::texture_node_builder::ImageNodesBuilder;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 480.0;

pub struct Game {
    resources: Resources,
    scene: Handle<Scene>,
    camera: Handle<Node>,
    // Root of all the object nodes; on start, this is a phony node.
    background: Handle<Node>,
    input: InputController,
    state: State,
    menu_state: MenuState,
    menu_num_players: u8,
    menu_difficulty: u8,
}

impl ImageNodesBuilder for Game {
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

impl GameState for Game {
    fn init(engine: &mut Engine) -> Self {
        preset_window(engine);

        let resources = Resources::load(&engine.resource_manager);

        let (scene, camera, background) = build_initial_scene(engine);

        let input = InputController::new();

        let state = State::Menu;
        let menu_state = MenuState::NumPlayers;

        Self {
            resources,
            scene,
            camera,
            background,
            input,
            state,
            menu_state,
            menu_num_players: 1,
            menu_difficulty: 0,
        }
    }

    fn on_tick(&mut self, engine: &mut Engine, _dt: f32, _control_flow: &mut ControlFlow) {
        use {MenuState::*, State::*};

        let scene = &mut engine.scenes[self.scene];

        // The simplest way to model a design that is as close a possible to a convenentional 2d game
        // library, is to use the background as root node, and to dynamically add each sprite to draw
        // as node.
        //
        scene.graph.remove_node(self.background);

        match &self.state {
            Menu => {
                let (image_i1, image_i2) = match &self.menu_state {
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

        use VirtualKeyCode::*;

        match &self.state {
            Menu => match &self.menu_state {
                NumPlayers => {
                    if self.input.is_key_just_pressed(Up) || self.input.is_key_just_pressed(Down) {
                        self.menu_num_players = 1 + self.menu_num_players % 2;
                    }
                }
                _ => {}
            },
            _ => {}
        }

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
