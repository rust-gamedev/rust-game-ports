use crate::{actor::Anchor, collide_actor::CollideActor, HEIGHT};

pub const GRAVITY_ACTOR_DEFAULT_ANCHOR: Anchor = Anchor::CentreBottom;

const MAX_FALL_SPEED: i32 = 10;

pub trait GravityActor: CollideActor {
    fn vel_y(&self) -> i32;
    fn vel_y_mut(&mut self) -> &mut i32;
    fn landed(&self) -> bool;
    fn landed_mut(&mut self) -> &mut bool;

    fn update(&mut self, detect: bool, grid: &[&str]) {
        // Apply gravity, without going over the maximum fall speed
        *self.vel_y_mut() = (self.vel_y() + 1).min(MAX_FALL_SPEED);

        // The detect parameter indicates whether we should check for collisions with blocks as we fall. Normally we
        // want this to be the case - hence why this parameter is optional, and is True by default. If the player is
        // in the process of losing a life, however, we want them to just fall out of the level, so False is passed
        // in this case.
        if detect {
            // Move vertically in the appropriate direction, at the appropriate speed
            if self.move_(0, self.vel_y().signum(), self.vel_y().abs(), grid) {
                // If move returned True, we must have landed on a block.
                // Note that move doesn't apply any collision detection when the player is moving up - only down
                *self.vel_y_mut() = 0;
                *self.landed_mut() = true;
            }

            if self.top() >= HEIGHT {
                // Fallen off bottom - reappear at top
                *self.y_mut() = 1;
            }
        } else {
            // Collision detection disabled - just update the Y coordinate without any further checks
            *self.y_mut() += self.vel_y();
        }
    }
}
