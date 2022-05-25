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
        node::Node,
        Scene,
    },
};

use crate::game::Game;
use crate::input_controller::InputController;
use crate::media::Media;
use crate::menu_state::MenuState;
use crate::state::State;
use crate::{controls::Controls, game};

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 480.0;

pub struct GameGlobal {
    resources: Media,
    scene: Handle<Scene>,
    camera: Handle<Node>,
    input: InputController,
    game: Game,
    state: State,
    menu_state: Option<MenuState>,
    menu_num_players: u8,
    menu_difficulty: u8,
}

impl GameState for GameGlobal {
    fn init(engine: &mut Engine) -> Self {
        Self::preset_window(engine);

        let mut scene = Scene::new();

        let camera = Self::build_camera(&mut scene);

        let resources = Media::new(&engine.resource_manager, &mut scene);

        let input = InputController::new();

        let game = Game::new(None, None, game::DEFAULT_DIFFICULTY);
        let state = State::Menu;
        let menu_state = Some(MenuState::NumPlayers);

        let scene_h = engine.scenes.add(scene);

        Self {
            resources,
            scene: scene_h,
            camera,
            input,
            game,
            state,
            menu_state,
            menu_num_players: 1,
            menu_difficulty: 0,
        }
    }

    fn on_tick(&mut self, engine: &mut Engine, _dt: f32, _control_flow: &mut ControlFlow) {
        let mut scene = &mut engine.scenes[self.scene];

        self.resources.clear_images(&mut scene);

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
    fn preset_window(engine: &Engine) {
        let window = engine.get_window();

        window.set_inner_size(PhysicalSize {
            width: WIDTH,
            height: HEIGHT,
        });

        window.set_resizable(false);
    }

    fn update(&mut self, engine: &mut Engine) {
        use VirtualKeyCode::*;
        use {MenuState::*, State::*};

        let mut scene = &mut engine.scenes[self.scene];

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
                        self.resources.play_sound(&mut scene, "move", &[]);
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
            Play => {
                // First player to 9 wins
                let max_score = self.game.teams.iter().map(|t| t.score).max().unwrap();

                if max_score == 9 && self.game.score_timer == 1 {
                    self.state = State::GameOver;
                } else {
                    self.game.update();
                }
            }
            GameOver => {
                if self.input.is_key_just_pressed(Space) {
                    // Switch to menu state, and create a new game object without a player
                    self.state = State::Menu;
                    self.menu_state = Some(MenuState::NumPlayers);
                    self.game = Game::new(None, None, game::DEFAULT_DIFFICULTY);
                }
            }
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

                self.resources
                    .draw_image(scene, "menu", &[image_i1, image_i2], 0, 0, 0);
            }
            Play => {
                //
            }
            GameOver => {
                //
            }
        }
    }

    // Returns (scene, camera, root node)
    //
    fn build_camera(scene: &mut Scene) -> Handle<Node> {
        CameraBuilder::new(BaseBuilder::new())
            .with_projection(Projection::Orthographic(OrthographicProjection {
                z_near: -0.1,
                z_far: 16.0,
                vertical_size: HEIGHT / 2.0,
            }))
            .build(&mut scene.graph)
    }
}
