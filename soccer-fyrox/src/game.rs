use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::prelude::*;

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
    players: Vec<RCC<Player>>,
    goals: Vec<Goal>,
    kickoff_player: Option<RCC<Player>>,
    ball: Ball,
    camera_focus: Vector2<i16>,
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
        let difficulty = DIFFICULTY[difficulty as usize];

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
        let goals = vec![];
        let kickoff_player = None;

        //# Create ball
        let ball = Ball::new();

        //# Focus camera on ball - copy ball pos
        let camera_focus = ball.vpos.clone();

        let mut instance = Self {
            teams,
            difficulty,
            score_timer,
            scoring_team,
            players,
            goals,
            kickoff_player,
            ball,
            camera_focus,
        };

        instance.reset();

        instance
    }

    fn reset(&mut self) {
        //# Called at game start, and after a goal has been scored

        // See Player#peer comment.
        //
        for player in &mut self.players {
            player.borrow_mut().peer = None;
        }

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
            self.players.push(Rc::new(RefCell::new(Player::new(
                random_offset(pos.0),
                random_offset(pos.1),
                0,
            ))));
            self.players.push(new_rcc(Player::new(
                random_offset(LEVEL_W - pos.0),
                random_offset(LEVEL_W - pos.1),
                1,
            )));
        }

        //# Players in the list are stored in an alternating fashion - a team 0 player, then a team 1 player, and so on.
        //# The peer for each player is the opposing team player at the opposite end of the list. As there are 14 players
        //# in total, the peers are 0 and 13, 1 and 12, 2 and 11, and so on.
        for (a, b) in self.players.iter().zip(self.players.iter().rev()) {
            a.borrow_mut().peer = Some(Rc::clone(b));
        }

        //# Create two goals
        self.goals = (0..2).into_iter().map(|i| Goal::new(i)).collect();

        //# The current active player under control by each team, indicated by arrows over their heads
        //# Choose first two players to begin with
        self.teams[0].active_control_player = Some(Rc::clone(&self.players[0]));
        self.teams[1].active_control_player = Some(Rc::clone(&self.players[1]));

        //# If team 1 just scored (or if it's the start of the game), team 0 will kick off
        let other_team = if self.scoring_team == 0 { 1 } else { 0 };

        //# Players are stored in the players list in an alternating fashion – the first player being on team 0, the
        //# second on team 1, the third on team 0 etc. The player that kicks off will always be the first player of
        //# the relevant team.
        self.kickoff_player = Some(Rc::clone(&self.players[other_team as usize]));

        //# Set pos of kickoff player. A team 0 player will stand to the left of the ball, team 1 on the right
        self.kickoff_player.as_ref().unwrap().borrow_mut().vpos =
            Vector2::new(HALF_LEVEL_W - 30 + other_team * 60, HALF_LEVEL_H);
    }

    pub fn update(&mut self) {
        // WRITEME
    }
}
