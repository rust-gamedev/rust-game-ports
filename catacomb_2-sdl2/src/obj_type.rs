use crate::{active_obj::activeobj, class_type::classtype, obj_def_type::objdeftype};

#[derive(Copy, Clone)]
pub struct objtype {
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
    pub filler2: [u8; 2],
}

impl objtype {
    pub fn update_from_activeobj(&mut self, active_o: &activeobj) {
        self.active = active_o.active;
        self.class = active_o.class;
        self.x = active_o.x;
        self.y = active_o.y;
        self.stage = active_o.stage;
        self.delay = active_o.delay;
        self.dir = active_o.dir;
        self.hp = active_o.hp;
        self.oldx = active_o.oldx;
        self.oldy = active_o.oldy;
        self.oldtile = active_o.oldtile;
        self.filler = active_o.filler;
    }

    // Updates fields from `think` onwards
    //
    pub fn update_from_objdeftype(&mut self, o: &objdeftype) {
        self.think = o.think;
        self.contact = o.contact;
        self.solid = o.solid;
        self.firstchar = o.firstchar;
        self.size = o.size;
        self.stages = o.stages;
        self.dirmask = o.dirmask;
        self.speed = o.speed;
        self.hitpoints = o.hitpoints;
        self.damage = o.damage;
        self.points = o.points;
        self.filler2 = o.filler;
    }
}
