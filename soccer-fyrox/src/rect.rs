pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn collidepoint(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x <= (self.x + self.width) && y <= (self.y + self.height)
    }
}
