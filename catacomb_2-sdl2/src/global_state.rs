use crate::{
    active_obj::activeobj,
    class_type::classtype::*,
    control_struct::ControlStruct,
    demo_enum::demoenum,
    dir_type::dirtype::*,
    exit_type::exittype::{self, *},
    obj_def_type::objdeftype,
    obj_type::objtype,
    state_type::statetype::{self, *},
    vec2::Vec2,
};

/// Includes most of the former globals.
#[rustfmt::skip]
pub struct GlobalState {
    // //////////////////////////////////////////////////////////
    // Rust port: shared
    // //////////////////////////////////////////////////////////

    pub gamexit: exittype,           /*determines what to do after playloop*/
    pub oldtiles: [i32; 576],        /*tile displayed last refresh*/
    pub background: [[i32; 86]; 87], /*base map*/
    pub view: [[i32; 86]; 87],       /*base map with objects drawn in*/
    pub origin: Vec2,                /*current world location of ul corn*/
    pub priority: [u8; 2048],        /*tile draw overlap priorities*/

    pub items: [i16; 6],
    pub shotpower: i32, /*0-13 characters in power meter*/
    pub side: i32,      /*which side shots come from*/

    pub o: [activeobj; 201], /*everything that moves is here*/
    pub obj: objtype,        /*total info about objecton and alt*/
    pub numobj: i32,         /*number of objects in o now*/
    pub objecton: i32,       // ^^

    pub objdef: [objdeftype; 23],

    pub playdone: bool,
    pub leveldone: bool,

    pub frameon: u16,

    pub exitdemo: bool,
    pub resetgame: bool,

    pub ctrl: ControlStruct,

    pub pics: Vec<u8>,

    // loaded into ES in the draw routines
    // should be adjusted after grmode
    // switches, page flipping, and scrolls
    pub screenseg: [u8; 64000],
    pub screencenter: Vec2,

    pub indemo: demoenum,

    // //////////////////////////////////////////////////////////
    // Rust port: Private to catacomb.rs
    // //////////////////////////////////////////////////////////

    pub saveitems: [i16; 6],
    pub saveo: [activeobj; 1], // ^^
    pub gamestate: statetype,
    pub savescore: i32,

    // //////////////////////////////////////////////////////////
    // Rust port: Private to cat_play.rs
    // //////////////////////////////////////////////////////////

    pub boltsleft: i32,  /*number of shots left in a bolt*/
    pub altobj: objtype, /*total info about objecton and alt*/
    pub altnum: i32,     /*o[#] of altobj*/
    pub chkx: i32,       /*spot being checked by walk*/
    pub chky: i32,       // ^^
    pub chkspot: i32,    // ^^
    pub GODMODE: bool,   // NOLAN ADDED

    // //////////////////////////////////////////////////////////
    // Rust port: unused
    // //////////////////////////////////////////////////////////

    // pub tempb: bool,
    // pub tempp: *mut i8,
    // pub grmem: *mut i8,
    // pub VGAPAL: i32,  // just to make pcrlib happy
    // EGADATASTART: u32
}

impl GlobalState {
    pub fn new(
        priority: [u8; 2048],
        items: [i16; 6],
        saveitems: [i16; 6],
        shotpower: i32,
        o: [activeobj; 201],
        saveo: [activeobj; 1],
        obj: objtype,
        altobj: objtype,
        altnum: i32,
        numobj: i32,
        objecton: i32,
        objdef: [objdeftype; 23],
        playdone: bool,
        leveldone: bool,
        chkx: i32,
        chky: i32,
        chkspot: i32,
        frameon: u16,
        exitdemo: bool,
        resetgame: bool,
        gamestate: statetype,
        ctrl: ControlStruct,
        pics: Vec<u8>,
        savescore: i32,
        GODMODE: bool,
        side: i32,
        boltsleft: i32,
        view: [[i32; 86]; 87],
        screencenter: Vec2,
        gamexit: exittype,
        oldtiles: [i32; 576],
        screenseg: [u8; 64000],
        background: [[i32; 86]; 87],
        origin: Vec2,
        indemo: demoenum,
    ) -> Self {
        Self {
            priority,
            items,
            saveitems,
            shotpower,
            o,
            saveo,
            obj,
            altobj,
            altnum,
            objecton,
            numobj,
            objdef,
            playdone,
            leveldone,
            chkx,
            chky,
            chkspot,
            frameon,
            exitdemo,
            resetgame,
            gamestate,
            ctrl,
            pics,
            savescore,
            GODMODE,
            side,
            boltsleft,
            view,
            screencenter,
            gamexit,
            oldtiles,
            screenseg,
            background,
            origin,
            indemo,
        }
    }
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new(
            [0; 2048],
            [0; 6],
            [0; 6],
            0,
            [activeobj {
                active: false,
                class: nothing,
                x: 0,
                y: 0,
                stage: 0,
                delay: 0,
                dir: 0,
                hp: 0,
                oldx: 0,
                oldy: 0,
                oldtile: 0,
                filler: [0; 1],
            }; 201],
            [activeobj {
                active: false,
                class: nothing,
                x: 0,
                y: 0,
                stage: 0,
                delay: 0,
                dir: 0,
                hp: 0,
                oldx: 0,
                oldy: 0,
                oldtile: 0,
                filler: [0; 1],
            }; 1],
            objtype {
                active: false,
                class: nothing,
                x: 0,
                y: 0,
                stage: 0,
                delay: 0,
                dir: 0,
                hp: 0,
                oldx: 0,
                oldy: 0,
                oldtile: 0,
                filler: [0; 1],
                think: 0,
                contact: 0,
                solid: 0,
                firstchar: 0,
                size: 0,
                stages: 0,
                dirmask: 0,
                speed: 0,
                hitpoints: 0,
                damage: 0,
                points: 0,
                filler2: [0; 2],
            },
            objtype {
                active: false,
                class: nothing,
                x: 0,
                y: 0,
                stage: 0,
                delay: 0,
                dir: 0,
                hp: 0,
                oldx: 0,
                oldy: 0,
                oldtile: 0,
                filler: [0; 1],
                think: 0,
                contact: 0,
                solid: 0,
                firstchar: 0,
                size: 0,
                stages: 0,
                dirmask: 0,
                speed: 0,
                hitpoints: 0,
                damage: 0,
                points: 0,
                filler2: [0; 2],
            },
            0,
            0,
            0,
            [objdeftype {
                think: 0,
                contact: 0,
                solid: 0,
                firstchar: 0,
                size: 0,
                stages: 0,
                dirmask: 0,
                speed: 0,
                hitpoints: 0,
                damage: 0,
                points: 0,
                filler: [0; 2],
            }; 23],
            false,
            false,
            0,
            0,
            0,
            0,
            false,
            false,
            ingame,
            ControlStruct {
                dir: north,
                button1: false,
                button2: false,
            },
            vec![],
            0,
            false,
            0,
            0,
            [[0; 86]; 87],
            Vec2::new(19, 11),
            quited,
            [0; 576],
            [8; 64000],
            [[0; 86]; 87],
            Vec2::new(0, 0),
            demoenum::notdemo,
        )
    }
}
