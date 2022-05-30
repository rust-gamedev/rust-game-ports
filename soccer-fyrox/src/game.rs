use fyrox::scene::Scene;
use rand::Rng;

use crate::{
    controls::Controls,
    difficulty::{self, Difficulty},
    game_global::LEVEL_W,
    media::Media,
    player::Player,
    team::Team,
};

pub const DEFAULT_DIFFICULTY: u8 = 2;
pub const PLAYER_START_POS: [(i16, i16); 7] = [
    (350, 550),
    (650, 450),
    (200, 850),
    (500, 750),
    (800, 950),
    (350, 1250),
    (650, 1150),
];

pub struct Game {
    pub teams: Vec<Team>,
    difficulty: Difficulty,
    pub score_timer: i32,
    scoring_team: u8,
    players: Vec<Player>,
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
            //# Beginning a game with at least 1 human player
            //# music.fadeout(1); // WRITEME: Fyrox doesn't currently support fading out
            media.stop_looping_sound(scene, "music/theme"); // ^^ remove once fadeout is implemented
            media.play_looping_sound(scene, "sounds/crowd");
            media.play_sound(scene, "sounds/start", &[]);
        } else {
            //# No players - we must be on the menu. Play title music.
            media.play_looping_sound(scene, "music/theme");
            media.stop_looping_sound(scene, "sounds/crowd");
        }

        let score_timer = 0;
        let scoring_team = 1;

        let players = vec![];

        let mut instance = Self {
            teams,
            difficulty,
            score_timer,
            scoring_team,
            players,
        };

        instance.reset();

        instance
    }

    fn reset(&mut self) {
        //# Called at game start, and after a goal has been scored

        //# Set up players list/positions
        //# The lambda function is used to give the player start positions a slight random offset so they're not
        //# perfectly aligned to their starting spots
        self.players.clear();
        // Watch out! Python's randint() spec is different, as it's inclusive on both ends, so we use
        // 33 on the right end.
        let random_offset = |x| x + rand::thread_rng().gen_range(-32..33);
        for pos in PLAYER_START_POS {
            //# pos is a pair of coordinates in a tuple
            //# For each entry in pos, create one player for each team - positions are flipped (both horizontally and
            //# vertically) versions of each other
            self.players
                .push(Player::new(random_offset(pos.0), random_offset(pos.1), 0));
            self.players.push(Player::new(
                random_offset(LEVEL_W - pos.0),
                random_offset(LEVEL_W - pos.1),
                1,
            ));
        }
    }

    pub fn update(&mut self) {
        // WRITEME
    }
}
