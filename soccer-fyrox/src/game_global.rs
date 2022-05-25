use fyrox::{
    core::{algebra::Vector3, pool::Handle},
    dpi::PhysicalSize,
    engine::framework::prelude::*,
    engine::Engine,
    event::{ElementState, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    scene::{
        base::BaseBuilder,
        camera::{CameraBuilder, OrthographicProjection, Projection},
        node::Node,
        pivot::PivotBuilder,
        sound::{
            SoundBuilder,
            Status::{self, Playing},
        },
        transform::TransformBuilder,
        Scene,
    },
};

use crate::input_controller::InputController;
use crate::menu_state::MenuState;
use crate::resources::Resources;
use crate::state::State;
use crate::{controls::Controls, game};
use crate::{game::Game, texture_node_builder::build_image_node};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 480.0;

pub struct GameGlobal {
    resources: Resources,
    scene: Handle<Scene>,
    camera: Handle<Node>,
    images_root: Handle<Node>,
    sounds_root: Handle<Node>,
    // There's only one music, so we use an Option; using a root node is an equally valid approach.
    music: Option<Handle<Node>>,
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

        let resources = Resources::load(&engine.resource_manager);

        let (scene, camera, images_root, sounds_root) = Self::build_initial_scene(engine);

        let music = None;

        let input = InputController::new();

        let game = Game::new(None, None, game::DEFAULT_DIFFICULTY);
        let state = State::Menu;
        let menu_state = Some(MenuState::NumPlayers);

        Self {
            resources,
            scene,
            camera,
            images_root,
            sounds_root,
            music,
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

        // The simplest way to model a design that is as close a possible to a conventional 2d game
        // library, is to a pivot node as root node, and to dynamically add each sprite to draw as node.
        // By scaling the pivot node to the screen size, we don't need to scale the sprites.
        //
        self.clear_scene(&mut scene);

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
                        self.play_sound(&mut scene, "move", &[]);
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

                self.draw_image(scene, "menu", &[image_i1, image_i2], 0, 0, 0);
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
    fn build_initial_scene(
        engine: &mut Engine,
    ) -> (Handle<Scene>, Handle<Node>, Handle<Node>, Handle<Node>) {
        let mut scene = Scene::new();

        let camera = CameraBuilder::new(BaseBuilder::new())
            .with_projection(Projection::Orthographic(OrthographicProjection {
                z_near: -0.1,
                z_far: 16.0,
                vertical_size: HEIGHT / 2.0,
            }))
            .build(&mut scene.graph);

        let images_root = PivotBuilder::new(
            BaseBuilder::new().with_local_transform(
                TransformBuilder::new()
                    .with_local_scale(Vector3::new(WIDTH, HEIGHT, f32::EPSILON))
                    .build(),
            ),
        )
        .build(&mut scene.graph);

        let sounds_root = PivotBuilder::new(BaseBuilder::new()).build(&mut scene.graph);

        let scene = engine.scenes.add(scene);

        (scene, camera, images_root, sounds_root)
    }

    fn clear_scene(&mut self, scene: &mut Scene) {
        for child in scene.graph[self.images_root].children().to_vec() {
            scene.graph.remove_node(child);
        }

        // Sounds still work if they're quickly (on the next frame) removed, but this is a cleaner approach.
        //
        for child in scene.graph[self.sounds_root].children().to_vec() {
            let sound = scene.graph[child].as_sound();

            if sound.status() == Status::Stopped {
                scene.graph.remove_node(child)
            }
        }
    }

    // Draws the image (loads the texture, adds the node to the scene, and links it to the root).
    // This is difficult to name, since the semantics of bevy and a 2d game are different.
    //
    fn draw_image(
        &mut self,
        scene: &mut Scene,
        base: &str,
        indexes: &[u8],
        x: i16,
        y: i16,
        z: i16,
    ) {
        let texture = self.resources.image(base, indexes);
        let background = build_image_node(&mut scene.graph, texture, x, y, z);
        scene.graph.link_nodes(background, self.images_root);
    }

    fn play_sound(&self, scene: &mut Scene, base: &str, indexes: &[u8]) {
        let base = "sounds/".to_string() + base;
        let sound = self.resources.sound(base, indexes);

        let sound_h = SoundBuilder::new(BaseBuilder::new())
            .with_buffer(Some(sound))
            .with_status(Playing)
            .build(&mut scene.graph);

        scene.graph.link_nodes(sound_h, self.sounds_root);
    }

    fn play_music(&mut self, scene: &mut Scene, base: &str) {
        if self.music.is_some() {
            panic!("There must be no music references, to play_music()");
        }

        let base = "music/".to_string() + base;
        let sound = self.resources.sound(base, &[]);

        self.music = Some(
            SoundBuilder::new(BaseBuilder::new())
                .with_buffer(Some(sound))
                .with_looping(true)
                .with_status(Playing)
                .build(&mut scene.graph),
        );
    }

    fn stop_music(&mut self, scene: &mut Scene) {
        let music_h = self
            .music
            .expect("A music reference must exist, to stop_music()!");

        scene.remove_node(music_h);
        self.music = None;
    }
}
