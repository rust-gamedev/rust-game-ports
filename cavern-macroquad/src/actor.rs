use macroquad::prelude::{draw_texture, Texture2D, WHITE};

#[derive(Clone, Copy)]
pub enum Anchor {
    Centre,
    CentreBottom,
}

// Rust: A private trait could be used, but it doesn't help much.
//
fn top_left_pos(anchor: Anchor, x: i32, y: i32, image: &Texture2D) -> (i32, i32) {
    let (image_width, image_height) = (image.width() as i32, image.height() as i32);

    let (diff_x, diff_y) = match anchor {
        Anchor::Centre => (image_width / 2, image_height / 2),
        Anchor::CentreBottom => (image_width / 2, image_height),
    };

    (x - diff_x, y - diff_y)
}

pub trait Actor {
    // Rust: We can't use a tuple, because we'll need mutable access to one field; Vec2 would work,
    // but it has f32 field. For simplicity, we separate x/y, which doesn't make any meaningful difference.
    fn x(&self) -> i32;
    fn x_mut(&mut self) -> &mut i32;
    fn y(&self) -> i32;
    fn y_mut(&mut self) -> &mut i32;
    fn image(&self) -> Texture2D;
    fn anchor(&self) -> Anchor;

    // Rust: All the geometry methods below are meant to be conveniently implemented, not fast.

    fn top(&self) -> i32 {
        let image = self.image();
        let top_left_pos = top_left_pos(self.anchor(), self.x(), self.y(), &image);

        top_left_pos.1
    }

    fn bottom(&self) -> i32 {
        self.top() + self.image().height() as i32
    }

    fn left(&self) -> i32 {
        let image = self.image();
        let top_left_pos = top_left_pos(self.anchor(), self.x(), self.y(), &image);

        top_left_pos.0
    }

    fn right(&self) -> i32 {
        self.left() + self.image().width() as i32
    }

    fn center(&self) -> (i32, i32) {
        let center_x = self.left() + self.image().width() as i32 / 2;
        let center_y = self.top() + self.image().height() as i32 / 2;

        (center_x, center_y)
    }

    fn collidepoint(&self, pos: (i32, i32)) -> bool {
        self.left() <= pos.0
            && pos.0 <= self.right()
            && self.top() <= pos.1
            && pos.1 <= self.bottom()
    }

    fn draw(&self) {
        let image = self.image();
        let top_left_pos = top_left_pos(self.anchor(), self.x(), self.y(), &image);

        draw_texture(image, top_left_pos.0 as f32, top_left_pos.1 as f32, WHITE);
    }
}
