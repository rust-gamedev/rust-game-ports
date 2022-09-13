#[derive(Clone, Copy)]
pub struct objdeftype {
    pub think: u8,
    pub contact: u8,
    pub solid: u8,
    pub firstchar: u16,
    pub size: u8,
    pub stages: u8,
    pub dirmask: u8,
    pub speed: u16,
    pub hitpoints: u8,
    pub damage: u8,
    pub points: u16,
    pub filler: [u8; 2],
}
