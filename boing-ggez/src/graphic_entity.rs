use ggez::graphics::{Canvas, DrawParam, Image};
use glam::Vec2;

/// Trait for implementing the drawing part of an Actor.
pub trait GraphicEntity {
    fn image(&self) -> &Image;
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    /// Draws an image, anchored to its center.
    /// This is due to ggez not supporting anchoring.
    fn draw(&mut self, canvas: &mut Canvas) {
        let dest = Vec2::new(
            self.x() - (self.image().width() / 2) as f32,
            self.y() - (self.image().height() / 2) as f32,
        );
        canvas.draw(self.image(), DrawParam::new().dest(dest));
    }
}
