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
            media.stop_looping_sound(scene, "theme"); // ^^ remove once fadeout is implemented
            media.play_looping_sound(scene, "crowd");
            media.play_sound(scene, "start", &[]);
        } else {
            //# No players - we must be on the menu. Play title music.
            media.play_looping_sound(scene, "theme");
            media.stop_looping_sound(scene, "crowd");
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

    pub fn draw(&self, scene: &mut Scene, media: &mut Media) {
        //# For the purpose of scrolling, all objects will be drawn with these offsets
        let offset_x = (self.camera_focus.x - WIDTH / 2).clamp(0, LEVEL_W - WIDTH); // max(0, min(LEVEL_W - WIDTH, self.camera_focus.x - WIDTH / 2));
        let offset_y = (self.camera_focus.y - HEIGHT / 2).clamp(0, LEVEL_H - HEIGHT);
        let offset = Vector2::new(offset_x, offset_y);

        media.draw_image(scene, "pitch", &[], -offset_x, -offset_y, 0, Anchor::Center);

        //# Prepare to draw all objects
        //# 1. Create a list of all players and the ball, sorted based on their Y positions
        //# 2. Add object shadows to the list
        //# 3. Add the two goals at each end of the list
        //# (note - technically we're not adding items to the list in steps two and three, we're creating a new list
        //# which consists of the old list plus the new items)

        // There are different approaches to modeling the Rust logic, although it's not worth mixing
        // the  `Goal`s in the iteration anyway.
        // One approach is to add a `shadow -> Option<BareActor>` function to `MyActor`.
        // This requires modifying the macro, because the fn needs to be a default in the trait, and
        // the macro needs to overwrite it when specified as macro parameter.
        // Another approach is to create a subtrait with this function, which doesn't require chaning
        // the macro.
        // The other (below) approach is to sort the players and find the ball index, then iterate while
        // testing the index.
        // Note that we could simplify and just draw players+shadows on a single cycle.

        self.goals[0].draw(scene, media, offset_x, offset_y);

        let mut sorted_players = self
            .players
            .iter()
            .map(|player| Rc::clone(&player))
            .collect::<Vec<_>>();

        sorted_players.sort_by(|a, b| (a.borrow().vpos.y.partial_cmp(&b.borrow().vpos.y).unwrap()));

        let ball_draw_i = sorted_players
            .iter()
            .enumerate()
            .find_map(|(i, p)| (self.ball.vpos().y < p.borrow().vpos().y).then_some(i))
            .unwrap_or(sorted_players.len());

        for i in 0..=sorted_players.len() {
            if i == ball_draw_i {
                self.ball.draw(scene, media, offset_x, offset_y);
            }

            if i < sorted_players.len() {
                sorted_players[i]
                    .borrow()
                    .draw(scene, media, offset_x, offset_y)
            }
        }

        for i in 0..=sorted_players.len() {
            if i == ball_draw_i {
                self.ball.shadow.draw(scene, media, offset_x, offset_y);
            }

            if i < sorted_players.len() {
                sorted_players[i]
                    .borrow()
                    .shadow
                    .draw(scene, media, offset_x, offset_y)
            }
        }

        self.goals[1].draw(scene, media, offset_x, offset_y);

        //# Show active players
        for t in 0..2 {
            //# Only show arrow for human teams
            if self.teams[t].human() {
                let arrow_pos = &self.teams[t]
                    .active_control_player
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .vpos()
                    - offset
                    - Vector2::new(11, 45);
                media.draw_image(
                    scene,
                    "arrow",
                    &[t as u8],
                    arrow_pos.x,
                    arrow_pos.y,
                    0,
                    Anchor::Center,
                );
            }
        }
    }
}
