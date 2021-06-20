use macroquad::prelude::{draw_texture, Texture2D, WHITE};

#[derive(Clone, Copy)]
pub enum Anchor {
    #[allow(dead_code)]
    Centre,
    CentreBottom,
}

pub trait Actor {
    // Rust: We can't use a tuple, because we'll need mutable access to one field; Vec2 would work,
    // but it has f32 field. For simplicity, we separate x/y, which doesn't make any meaningful difference.
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn image(&self) -> Texture2D;
    fn anchor(&self) -> Anchor;

    fn draw(&self) {
        let image = self.image();

        let (diff_x, diff_y) = match self.anchor() {
            Anchor::Centre => (image.width() / 2., image.height() / 2.),
            Anchor::CentreBottom => (image.width() / 2., image.height()),
        };

        draw_texture(
            image,
            self.x() as f32 - diff_x,
            self.y() as f32 - diff_y,
            WHITE,
        );
    }
}
