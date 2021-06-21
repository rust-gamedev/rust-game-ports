use crate::{
    actor::{Actor, Anchor},
    GRID_BLOCK_SIZE, LEVEL_X_OFFSET, NUM_COLUMNS, NUM_ROWS,
};

pub const COLLIDE_ACTOR_DEFAULT_ANCHOR: Anchor = Anchor::Centre;

fn block(x: i32, y: i32, grid: &[&str]) -> bool {
    // Is there a level grid block at these coordinates?
    let grid_x = (x - LEVEL_X_OFFSET) / GRID_BLOCK_SIZE;
    let grid_y = y / GRID_BLOCK_SIZE;
    if grid_y > 0 && grid_y < NUM_ROWS {
        let row = grid[grid_y as usize];
        grid_x >= 0
            && grid_x < NUM_COLUMNS
            && row.len() > 0
            && row.as_bytes()[grid_x as usize] != ' ' as u8
    } else {
        false
    }
}

pub trait CollideActor: Actor {
    fn move_(&mut self, dx: i32, dy: i32, speed: i32, grid: &[&str]) -> bool {
        let (mut new_x, mut new_y) = (self.x(), self.y());

        // Movement is done 1 pixel at a time, which ensures we don't get embedded into a wall we're moving towards
        for _ in 0..speed {
            new_x += dx;
            new_y += dy;

            if new_x < 70 || new_x > 730 {
                // Collided with edge of level
                return true;
            }

            // Normally you don't need brackets surrounding the condition for an if statement (unlike many other
            // languages), but in the case where the condition is split into multiple lines, using brackets removes
            // the need to use the \ symbol at the end of each line.
            // The code below checks to see if we're position we're trying to move into overlaps with a block. We only
            // need to check the direction we're actually moving in. So first, we check to see if we're moving down
            // (dy > 0). If that's the case, we then check to see if the proposed new y coordinate is a multiple of
            // GRID_BLOCK_SIZE. If it is, that means we're directly on top of a place where a block might be. If that's
            // also true, we then check to see if there is actually a block at the given position. If there's a block
            // there, we return True and don't update the object to the new position.
            // For movement to the right, it's the same except we check to ensure that the new x coordinate is a multiple
            // of GRID_BLOCK_SIZE. For moving left, we check to see if the new x coordinate is the last (right-most)
            // pixel of a grid block.
            // Note that we don't check for collisions when the player is moving up.
            if (dy > 0 && new_y % GRID_BLOCK_SIZE == 0
                || dx > 0 && new_x % GRID_BLOCK_SIZE == 0
                || dx < 0 && new_x % GRID_BLOCK_SIZE == GRID_BLOCK_SIZE - 1)
                && block(new_x, new_y, grid)
            {
                return true;
            }

            // We only update the object's position if there wasn't a block there.
            *self.x_mut() = new_x;
            *self.y_mut() = new_y;
        }

        // Didn't collide with block or edge of level
        false
    }
}
