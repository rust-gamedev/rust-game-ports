use ggez::audio::{self, SoundSource};
use ggez::graphics::{DrawParam, Drawable, Image};
use ggez::{Context, GameResult};
use glam::Vec2;

use crate::ball::Ball;
use crate::bat::Bat;
use crate::impact::Impact;
use crate::WINDOW_WIDTH;

pub struct Game {
    pub bats: [Bat; 2],
    pub ball: Ball,

    /// List of the current impacts to display.
    pub impacts: Vec<Impact>,

    /// Offset added to the AI player's target Y position, so it won't aim to hit the ball exactly in
    /// the centre of the bat.
    pub ai_offset: f32,

    pub table_image: Image,
    pub effect_images: Vec<Image>,
    // Same technical considerations As Bat#images.
    pub digit_images: Vec<Vec<Image>>,

    pub score_goal_sound: audio::Source,
}

impl Game {
    pub fn new(
        context: &mut Context,
        controls: (
            Option<fn(&Context, &Ball, f32, &Bat) -> f32>,
            Option<fn(&Context, &Ball, f32, &Bat) -> f32>,
        ),
    ) -> Self {
        let table_image = Image::new(context, "/table.png").unwrap();
        let effect_images = (0..2)
            .map(|image_i| {
                let image_name = format!("/effect{}.png", image_i);
                Image::new(context, image_name).unwrap()
            })
            .collect();
        let digit_images = (0..3)
            .map(|player| {
                (0..=9)
                    .map(|image_i| {
                        let image_name = format!("/digit{}{}.png", player, image_i);
                        Image::new(context, image_name).unwrap()
                    })
                    .collect()
            })
            .collect();

        let score_goal_sound = audio::Source::new(context, "/score_goal0.ogg").unwrap();

        Self {
            bats: [
                Bat::new(context, 0, controls.0),
                Bat::new(context, 1, controls.1),
            ],
            ball: Ball::new(context, -1.),

            impacts: vec![],

            ai_offset: 0.,

            table_image,
            effect_images,
            digit_images,

            score_goal_sound,
        }
    }

    pub fn update(&mut self, context: &mut Context) -> GameResult {
        // Update all active objects
        for bat in &mut self.bats {
            bat.update(context, &self.ball, self.ai_offset)?
        }
        self.ball.update(
            context,
            &mut self.bats,
            &mut self.impacts,
            &mut self.ai_offset,
        )?;
        for impact in &mut self.impacts {
            impact.update(context)?
        }

        // Remove any expired impact effects from the list.
        // Interesting, this is easier in Rust :)
        self.impacts.retain(|impact| impact.time < 10);

        // Has ball gone off the left or right edge of the screen?
        if self.ball.out() {
            // Work out which player gained a point, based on whether the ball
            // was on the left or right-hand side of the screen
            let scoring_player = if self.ball.x < WINDOW_WIDTH { 1 } else { 0 };
            let losing_player = 1 - scoring_player;

            // We use the timer of the player who has just conceded a point to decide when to create a new ball in the
            // centre of the level. This timer starts at zero at the beginning of the game and counts down by one every
            // frame. Therefore, on the frame where the ball first goes off the screen, the timer will be less than zero.
            // We set it to 20, which means that this player's bat will display a different animation frame for 20
            // frames, and a new ball will be created after 20 frames
            if self.bats[losing_player].timer < 0 {
                self.bats[scoring_player].score += 1;

                self.score_goal_sound.play(context)?;

                self.bats[losing_player].timer = 20;
            } else if self.bats[losing_player].timer == 0 {
                // After 20 frames, create a new ball, heading in the direction of the player who just missed the ball
                let direction = if losing_player == 0 { -1. } else { 1. };
                self.ball = Ball::new(context, direction);
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult {
        // Draw background
        self.table_image.draw(context, DrawParam::new())?;

        // Draw 'just scored' effects, if required
        for (p, bat) in self.bats.iter().enumerate() {
            if bat.timer > 0 && self.ball.out() {
                self.effect_images[p].draw(context, DrawParam::new())?;
            }
        }

        // Draw bats, ball and impact effects - in that order.
        // The Rust design of this application doesn't include a common Actor trait, so we can't lump
        // the objects together and iterate them, but for this simplification only, it's not worth.

        for bat in &mut self.bats {
            bat.draw(context)?;
        }

        self.ball.draw(context)?;

        for impact in &mut self.impacts {
            impact.draw(context)?;
        }

        // Display scores - outer loop goes through each player
        for (p, bat) in self.bats.iter().enumerate() {
            // Convert score into a string of 2 digits (e.g. "05") so we can later get the individual digits
            let score = format!("{:02}", bat.score);

            // Inner loop goes through each digit
            for (i, score_char) in score.chars().enumerate() {
                let other_p = 1 - p;

                // Digit sprites are numbered 00 to 29, where the first digit is the colour (0 = grey,
                // 1 = blue, 2 = green) and the second digit is the digit itself
                // Colour is usually grey but turns red or green (depending on player number) when a
                // point has just been scored
                let colour = if self.bats[other_p].timer > 0 && self.ball.out() {
                    if p == 0 {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                };

                // There are different approaches to this. This is the simplest.
                let score_char_val = score_char.to_digit(10).unwrap() as usize;

                self.digit_images[colour][score_char_val].draw(
                    context,
                    DrawParam::new().dest(Vec2::new((255 + (160 * p) + (i * 55)) as f32, 46.)),
                )?;
            }
        }

        Ok(())
    }
}
