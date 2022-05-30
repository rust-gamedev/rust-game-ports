use fyrox::scene::Scene;

use crate::{
    controls::Controls,
    difficulty::{self, Difficulty},
    media::Media,
    team::Team,
};

pub const DEFAULT_DIFFICULTY: u8 = 2;

pub struct Game {
    pub teams: Vec<Team>,
    difficulty: Difficulty,
    pub score_timer: i32,
    scoring_team: u8,
}

impl Game {
    pub fn new(
        p1_controls: Option<Controls>,
        p2_controls: Option<Controls>,
        difficulty: u8,
        scene: &mut Scene,
        media: &mut Media,
    ) -> Self {
        let teams = vec![Team::new(p1_controls), Team::new(p2_controls)];
        let difficulty = difficulty::DIFFICULTY[difficulty as usize];

        if teams[0].human() {
            // Beginning a game with at least 1 human player
            // music.fadeout(1); // WRITEME: Fyrox doesn't currently support fading out
            media.stop_looping_sound(scene, "music/theme"); // ^^ remove once fadeout is implemented
            media.play_looping_sound(scene, "sounds/crowd");
            media.play_sound(scene, "sounds/start", &[]);
        } else {
            // No players - we must be on the menu. Play title music.
            media.play_looping_sound(scene, "music/theme");
            media.stop_looping_sound(scene, "sounds/crowd");
        }

        let score_timer = 0;
        let scoring_team = 1;

        let mut instance = Self {
            teams,
            difficulty,
            score_timer,
            scoring_team,
        };

        instance.reset();

        instance
    }

    fn reset(&mut self) {
        // WRITEME
    }

    pub fn update(&mut self) {
        // WRITEME
    }
}
