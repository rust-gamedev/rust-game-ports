use ggez::{graphics::Image, Context, GameResult};

use crate::{ball::Ball, controls::ai, graphic_entity::GraphicEntity, HALF_HEIGHT};

pub struct Bat {
    pub x: f32,
    pub y: f32,
    /// Player number
    pub player: i8,
    // 2^16 points out to be enough for anybody.
    pub score: u16,
    /// move_func is a function we may or may not have been passed by the code which created this
    /// object. If this bat is meant to be player controlled, move_func will be a function that when
    /// called, returns a number indicating the direction and speed in which the bat should move,
    /// based on the keys the player is currently pressing.
    /// If move_func is None, this indicates that this bat should instead be controlled by the AI method.
    pub move_func: fn(&Context, &Ball, f32, &Bat) -> f32,

    /// Each bat has a timer which starts at zero and counts down by one every frame. When a player
    /// concedes a point, their timer is set to 20, which causes the bat to display a different animation
    /// frame. It is also used to decide when to create a new ball in the centre of the screen - see
    /// comments in Game.update for more on this. Finally, it is used in Game.draw to determine when
    /// to display a visual effect over the top of the background.
    pub timer: i32,

    // Image index for the current player.
    pub current_image: usize,
    // Although these are arrays [of fixed size], which is also semantically more precise, working with
    // arrays can be cumbersome (ie. from iterators), so just use `Vec`.
    pub images: Vec<Vec<Image>>,
}

impl GraphicEntity for Bat {
    fn image(&self) -> &Image {
        &self.images[self.player as usize][self.current_image]
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Bat {
    pub fn new(
        context: &mut Context,
        player: i8,
        move_func: Option<fn(&Context, &Ball, f32, &Bat) -> f32>,
    ) -> Self {
        let x = if player == 0 { 40. } else { 760. };

        let move_func = move_func.unwrap_or(ai);

        let images = (0..2)
            .map(|player| {
                (0..3)
                    .map(|image_i| {
                        let image_name = format!("/bat{}{}.png", player, image_i);
                        Image::new(context, image_name).unwrap()
                    })
                    .collect()
            })
            .collect();

        Self {
            x: x,
            y: HALF_HEIGHT,
            player,
            score: 0,
            move_func,

            timer: 0,

            current_image: 0,
            images,
        }
    }

    pub fn update(&mut self, context: &mut Context, ball: &Ball, ai_offset: f32) -> GameResult {
        self.timer -= 1;
        // Our movement function tells us how much to move on the Y axis
        let y_movement = (self.move_func)(context, ball, ai_offset, self);

        // keycode, ball_pos, ai_offset, bat_pos

        // Apply y_movement to y position, ensuring bat does not go through the side walls
        self.y = (self.y + y_movement).clamp(80., 400.);

        // Choose the appropriate sprite. There are 3 sprites per player - e.g. bat00 is the left-hand player's
        // standard bat sprite, bat01 is the sprite to use when the ball has just bounced off the bat, and bat02
        // is the sprite to use when the bat has just missed the ball and the ball has gone out of bounds.
        // bat10, 11 and 12 are the equivalents for the right-hand player

        let frame = if self.timer > 0 {
            if ball.out() {
                2
            } else {
                1
            }
        } else {
            0
        };

        self.current_image = frame;

        Ok(())
    }
}
