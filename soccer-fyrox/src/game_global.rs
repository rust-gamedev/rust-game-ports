use std::env;

use fyrox::{
    dpi::PhysicalSize,
    engine::framework::prelude::GameState,
    engine::Engine,
    event::{ElementState, WindowEvent},
    event_loop::ControlFlow,
    scene::camera::{CameraBuilder, OrthographicProjection, Projection},
};

use crate::prelude::*;

const DEFAULT_WIN_SCORE: &str = "9";

pub struct GameGlobal {
    media: Media,
    scene: Handle<Scene>,
    camera: Handle<Node>,
    input: InputController,
    game: Game,
    state: State,
    menu_state: Option<MenuState>,
    menu_num_players: u8,
    menu_difficulty: u8,

    // For debugging; can be set via env var `SOCCER_WIN_SCORE`.
    win_score: u8,
}

impl GameState for GameGlobal {
    fn init(engine: &mut Engine) -> Self {
        Self::preset_window(engine);

        let mut scene = Scene::new();

        let camera = Self::add_camera(&mut scene);

        let mut media = Media::new(&engine.resource_manager);

        let input = InputController::new();

        let game = Game::new(None, None, DEFAULT_DIFFICULTY, &mut scene, &mut media);
        let state = State::Menu;
        let menu_state = Some(MenuState::NumPlayers);

        let scene_h = engine.scenes.add(scene);

        let win_score = env::var("SOCCER_WIN_SCORE")
            .unwrap_or(String::from(DEFAULT_WIN_SCORE))
            .parse()
            .unwrap();

        Self {
            media,
            scene: scene_h,
            camera,
            input,
            game,
            state,
            menu_state,
            menu_num_players: 1,
            menu_difficulty: 0,
            win_score,
        }
    }

    fn on_tick(&mut self, engine: &mut Engine, _dt: f32, _control_flow: &mut ControlFlow) {
        let mut scene = &mut engine.scenes[self.scene];
        let mut user_interface = &mut engine.user_interface;

        self.media.clear_images(&mut scene, &mut user_interface);

        self.update(engine);

        self.draw(engine, self.camera);

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

        // WATCH OUT! Don't invert this and the following, otherwise, resize won't work.
        // See https://#github.com/rust-windowing/winit/issues/2306.
        //
        window.set_resizable(false);

        window.set_inner_size(PhysicalSize {
            width: WIDTH,
            height: HEIGHT,
        });
    }

    fn update(&mut self, engine: &mut Engine) {
        use VirtualKeyCode::*;
        use {MenuState::*, State::*};

        let mut scene = &mut engine.scenes[self.scene];

        match &self.state {
            Menu => {
                if self.input.is_key_just_pressed(Space) {
                    if let Some(NumPlayers) = self.menu_state {
                        //# If we're doing a 2 player game, skip difficulty selection
                        if self.menu_num_players == 1 {
                            self.menu_state = Some(MenuState::Difficulty);
                        } else {
                            //# Start 2P game
                            self.state = State::Play;
                            self.menu_state = None;
                            self.game = Game::new(
                                Some(Controls::new(0)),
                                Some(Controls::new(1)),
                                DEFAULT_DIFFICULTY,
                                &mut scene,
                                &mut self.media,
                            )
                        }
                    } else {
                        //# Start 1P game
                        self.state = State::Play;
                        self.menu_state = None;
                        self.game = Game::new(
                            Some(Controls::new(0)),
                            None,
                            self.menu_difficulty,
                            &mut scene,
                            &mut self.media,
                        );
                    }
                } else {
                    //# Detect + act on up/down arrow keys
                    let mut selection_change: i8 = 0;
                    if self.input.is_key_just_pressed(Down) {
                        selection_change = 1
                    } else if self.input.is_key_just_pressed(Up) {
                        selection_change = -1;
                    }
                    if selection_change != 0 {
                        self.media.play_sound(&mut scene, "move", &[]);
                        if let Some(MenuState::NumPlayers) = self.menu_state {
                            self.menu_num_players = if self.menu_num_players == 1 { 2 } else { 1 };
                        } else {
                            self.menu_difficulty =
                                (self.menu_difficulty as i8 + selection_change).rem_euclid(3) as u8
                        }
                    }
                }

                self.game.update(&self.media, scene, &self.input)
            }
            Play => {
                //# First player to 9 wins
                let max_score = self.game.teams.iter().map(|t| t.score).max().unwrap();

                if self.win_score == 0
                    || (max_score == self.win_score && self.game.score_timer == 1)
                {
                    self.state = State::GameOver;
                } else {
                    self.game.update(&self.media, scene, &self.input);
                }
            }
            GameOver => {
                if self.input.is_key_just_pressed(Space) {
                    //# Switch to menu state, and create a new game object without a player
                    self.state = State::Menu;
                    self.menu_state = Some(MenuState::NumPlayers);
                    self.game =
                        Game::new(None, None, DEFAULT_DIFFICULTY, &mut scene, &mut self.media);
                }
            }
        }
    }

    fn draw(&mut self, engine: &mut Engine, camera: Handle<Node>) {
        let scene = &mut engine.scenes[self.scene];

        self.game.draw(scene, camera, &mut self.media);

        use {MenuState::*, State::*};

        match &self.state {
            Menu => {
                let (image_i1, image_i2) = match self.menu_state.as_ref().unwrap() {
                    NumPlayers => (0, self.menu_num_players),
                    Difficulty => (1, self.menu_difficulty),
                };

                self.media.draw_gui_image(
                    &mut engine.user_interface,
                    "menu",
                    &[image_i1, image_i2],
                    0.,
                    0.,
                );
            }
            Play => {
                //# Display score bar at top
                self.media.draw_gui_image(
                    &mut engine.user_interface,
                    "bar",
                    &[],
                    HALF_WINDOW_W - 176.,
                    0.,
                );
                //# Show score for each team
                for i in 0..2 {
                    self.media.draw_gui_image(
                        &mut engine.user_interface,
                        "s",
                        &[self.game.teams[i].score],
                        HALF_WINDOW_W + 7. - 39. * (i as f32),
                        6.,
                    );
                }

                //# Show GOAL image if a goal has recently been scored
                if self.game.score_timer > 0 {
                    self.media.draw_gui_image(
                        &mut engine.user_interface,
                        "goal",
                        &[],
                        HALF_WINDOW_W - 300.,
                        HEIGHT / 2. - 88.,
                    );
                }
            }
            GameOver => {
                //# Display "Game Over" image
                let index = (self.game.teams[1].score > self.game.teams[0].score) as u8;
                self.media
                    .draw_gui_image(&mut engine.user_interface, "over", &[index], 0., 0.);

                //# Show score for each team
                for i in 0..2 {
                    self.media.draw_gui_image(
                        &mut engine.user_interface,
                        "l",
                        &[i as u8, self.game.teams[i as usize].score],
                        HALF_WINDOW_W + 25. - 125. * i as f32,
                        144.,
                    );
                }
            }
        }
    }

    fn add_camera(scene: &mut Scene) -> Handle<Node> {
        CameraBuilder::new(BaseBuilder::new())
            .with_projection(Projection::Orthographic(OrthographicProjection {
                z_near: CAMERA_NEAR_Z,
                z_far: CAMERA_FAR_Z,
                vertical_size: (HEIGHT / 2.),
            }))
            .build(&mut scene.graph)
    }
}
