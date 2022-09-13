use serdine::derive::{Deserialize, Serialize};

use crate::{class_type::classtype, obj_type::objtype};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct activeobj {
    pub active: bool,
    pub class: classtype,
    pub x: u8,
    pub y: u8,
    pub stage: u8,
    pub delay: u8,
    pub dir: u16,
    pub hp: i8,
    pub oldx: u8,
    pub oldy: u8,
    pub oldtile: i16,
    pub filler: [u8; 1],
}

impl From<objtype> for activeobj {
    fn from(source: objtype) -> Self {
        Self {
            active: source.active,
            class: source.class,
            x: source.x,
            y: source.y,
            stage: source.stage,
            delay: source.delay,
            dir: source.dir,
            hp: source.hp,
            oldx: source.oldx,
            oldy: source.oldy,
            oldtile: source.oldtile,
            filler: source.filler,
        }
    }
}
