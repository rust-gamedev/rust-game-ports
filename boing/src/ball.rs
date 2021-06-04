use ggez::{
    audio,
    graphics::{DrawParam, Drawable, Image},
    Context, GameResult,
};
use glam::Vec2;

use crate::{
    bat::Bat,
    impact::Impact,
    sounds_playback::{play_in_game_random_sound, play_in_game_sound},
    state::State,
    HALF_HEIGHT, HALF_WIDTH, WINDOW_WIDTH,
};

pub struct Ball {
    pub x: f32,
    pub y: f32,

    /// dx and dy together describe the direction in which the ball is moving. For example, if dx and
    /// dy are 1 and 0, the ball is moving to the right, with no movement up or down. If both values
    /// are negative, the ball is moving left and up, with the angle depending on the relative values
    /// of the two variables. If you're familiar with vectors, dx and dy represent a unit vector. If
    /// you're not familiar with vectors, see the explanation in the book.
    pub dx: f32,
    pub dy: f32,

    pub speed: u8,

    pub image: Image,

    pub hit_sounds: Vec<audio::Source>,
    pub hit_slow_sound: audio::Source,
    pub hit_medium_sound: audio::Source,
    pub hit_fast_sound: audio::Source,
    pub hit_veryfast_sound: audio::Source,
    pub bounce_sounds: Vec<audio::Source>,
    pub bounce_synth_sound: audio::Source,
}

impl Ball {
    pub fn new(context: &mut Context, dx: f32) -> Self {
        let image = Image::new(context, "/ball.png").unwrap();
        let hit_sounds = (0..5)
            .map(|i| {
                let sound_name = format!("/hit{}.ogg", i);
                audio::Source::new(context, sound_name).unwrap()
            })
            .collect();
        let hit_slow_sound = audio::Source::new(context, "/hit_slow0.ogg").unwrap();
        let hit_medium_sound = audio::Source::new(context, "/hit_medium0.ogg").unwrap();
        let hit_fast_sound = audio::Source::new(context, "/hit_fast0.ogg").unwrap();
        let hit_veryfast_sound = audio::Source::new(context, "/hit_veryfast0.ogg").unwrap();
        let bounce_sounds = (0..5)
            .map(|i| {
                let sound_name = format!("/bounce{}.ogg", i);
                audio::Source::new(context, sound_name).unwrap()
            })
            .collect();
        let bounce_synth_sound = audio::Source::new(context, "/bounce_synth0.ogg").unwrap();

        Self {
            x: HALF_WIDTH,
            y: HALF_HEIGHT,

            dx,
            dy: 0.,

            speed: 5,

            image,

            hit_sounds,
            hit_slow_sound,
            hit_medium_sound,
            hit_fast_sound,
            hit_veryfast_sound,
            bounce_sounds,
            bounce_synth_sound,
        }
    }

    pub fn update(
        &mut self,
        context: &mut Context,
        bats: &mut [Bat],
        impacts: &mut Vec<Impact>,
        ai_offset: &mut f32,
        state: State,
    ) -> GameResult {
        // Each frame, we move the ball in a series of small steps - the number of steps being based
        // on its speed attribute
        for _ in 0..self.speed {
            // Store the previous x position
            let original_x = self.x;

            // Move the ball based on dx and dy
            self.x += self.dx;
            self.y += self.dy;

            // Check to see if ball needs to bounce off a bat

            // To determine whether the ball might collide with a bat, we first measure the horizontal
            // distance from the ball to the centre of the screen, and check to see if its edge has
            // gone beyond the edge of the bat. The centre of each bat is 40 pixels from the edge of
            // the screen, or to put it another way, 360 pixels from the centre of the screen. The bat
            // is 18 pixels wide and the ball is 14 pixels wide. Given that these sprites are anchored
            // from their centres, when determining if they overlap or touch, we need to look at their
            // half-widths - 9 and 7. Therefore, if the centre of the ball is 344 pixels from the centre
            // of the screen, it can bounce off a bat (assuming the bat is in the right position on
            // the Y axis - checked shortly afterwards).
            // We also check the previous X position to ensure that this is the first frame in which
            // the ball crossed the threshold.

            if (self.x - HALF_WIDTH).abs() >= 344. && (original_x - HALF_WIDTH).abs() < 344. {
                // Now that we know the edge of the ball has crossed the threshold on the x-axis, we
                // need to check to see if the bat on the relevant side of the arena is at a suitable
                // position on the y-axis for the ball collide with it.

                let (new_dir_x, bat) = if self.x < HALF_WIDTH {
                    (1., &mut bats[0])
                } else {
                    (-1., &mut bats[1])
                };

                let difference_y = self.y - bat.y;

                if difference_y > -64. && difference_y < 64. {
                    // Ball has collided with bat - calculate new direction vector

                    // To understand the maths used below, we first need to consider what would happen with this kind of
                    // collision in the real world. The ball is bouncing off a perfectly vertical surface. This makes for a
                    // pretty simple calculation. Let's take a ball which is travelling at 1 metre per second to the right,
                    // and 2 metres per second down. Imagine this is taking place in space, so gravity isn't a factor.
                    // After the ball hits the bat, it's still going to be moving at 2 m/s down, but it's now going to be
                    // moving 1 m/s to the left instead of right. So its speed on the y-axis hasn't changed, but its
                    // direction on the x-axis has been reversed. This is extremely easy to code - "self.dx = -self.dx".
                    // However, games don't have to perfectly reflect reality.
                    // In Pong, hitting the ball with the upper or lower parts of the bat would make it bounce diagonally
                    // upwards or downwards respectively. This gives the player a degree of control over where the ball
                    // goes. To make for a more interesting game, we want to use realistic physics as the starting point,
                    // but combine with this the ability to influence the direction of the ball. When the ball hits the
                    // bat, we're going to deflect the ball slightly upwards or downwards depending on where it hit the
                    // bat. This gives the player a bit of control over where the ball goes.

                    // Bounce the opposite way on the X axis
                    self.dx = -self.dx;

                    // Deflect slightly up or down depending on where ball hit bat
                    self.dy += difference_y / 128.;

                    // Limit the Y component of the vector so we don't get into a situation where the ball is bouncing
                    // up and down too rapidly
                    self.dy = self.dy.clamp(-1., 1.);

                    // Ensure our direction vector is a unit vector, i.e. represents a distance of the equivalent of
                    // 1 pixel regardless of its angle
                    let normalised_d = Vec2::new(self.dx, self.dy).normalize();
                    self.dx = normalised_d.x;
                    self.dy = normalised_d.y;

                    // Create an impact effect
                    impacts.push(Impact::new(context, self.x - new_dir_x * 10., self.y));

                    // Increase speed with each hit
                    self.speed += 1;

                    // Add an offset to the AI player's target Y position, so it won't aim to hit the ball exactly
                    // in the centre of the bat
                    *ai_offset = fastrand::i32(-10..10) as f32;

                    // Bat glows for 10 frames
                    bat.timer = 10;

                    play_in_game_random_sound(context, state, &mut self.hit_sounds)?;

                    if self.speed <= 10 {
                        play_in_game_sound(context, state, &mut self.hit_slow_sound)?;
                    } else if self.speed <= 12 {
                        play_in_game_sound(context, state, &mut self.hit_medium_sound)?;
                    } else if self.speed <= 16 {
                        play_in_game_sound(context, state, &mut self.hit_fast_sound)?;
                    } else {
                        play_in_game_sound(context, state, &mut self.hit_veryfast_sound)?;
                    }
                }
            }

            // The top and bottom of the arena are 220 pixels from the centre
            if (self.y - HALF_HEIGHT).abs() > 220. {
                // Invert vertical direction and apply new dy to y so that the ball is no longer overlapping with the
                // edge of the arena
                self.dy = -self.dy;
                self.y += self.dy;

                // Create impact effect
                impacts.push(Impact::new(context, self.x, self.y));

                // Sound effect
                play_in_game_random_sound(context, state, &mut self.bounce_sounds)?;
                play_in_game_sound(context, state, &mut self.bounce_synth_sound)?;
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, context: &mut Context) -> GameResult {
        self.image
            .draw(context, DrawParam::new().dest(Vec2::new(self.x, self.y)))
    }

    pub fn out(&self) -> bool {
        // Has ball gone off the left or right edge of the screen?
        self.x < 0. || self.x > WINDOW_WIDTH
    }
}
