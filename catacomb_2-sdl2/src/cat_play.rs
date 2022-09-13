use crate::{
    active_obj::activeobj,
    catacomb::{clearold, dofkeys, loadlevel, refresh, restore},
    catasm::{doall, drawobj, eraseobj},
    class_type::classtype::*,
    control_struct::ControlStruct,
    cpanel_state::CpanelState,
    demo_enum::demoenum::*,
    dir_type::dirtype::{self, *},
    exit_type::exittype::*,
    extra_constants::maxobj,
    global_state::GlobalState,
    pcrlib_a::{drawchar, initrndt, rndt, PlaySound, WaitEndSound, WaitVBL},
    pcrlib_a_state::PcrlibAState,
    pcrlib_c::{
        centerwindow, get, ControlPlayer, UpdateScreen, _inputint, bioskey, clearkeys, print,
        print_str, RecordDemo, SaveDemo,
    },
    pcrlib_c_state::PcrlibCState,
    scan_codes::*,
    sdl_manager::SdlManager,
    tag_type::tagtype::*,
};

#[rustfmt::skip]
const altmeters: [[u8; 13]; 14] = [
    [127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [23,  127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [23,  25,  127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [23,  24,  25,  127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [23,  24,  24,  25,  127, 127, 127, 127, 127, 127, 127, 127, 127],
    [23,  24,  24,  24,  25,  127, 127, 127, 127, 127, 127, 127, 127],
    [23,  24,  24,  24,  24,  25,  127, 127, 127, 127, 127, 127, 127],
    [23,  24,  24,  24,  24,  24,  25,  127, 127, 127, 127, 127, 127],
    [23,  24,  24,  24,  24,  24,  24,  25,  127, 127, 127, 127, 127],
    [23,  24,  24,  24,  24,  24,  24,  24,  25,  127, 127, 127, 127],
    [23,  24,  24,  24,  24,  24,  24,  24,  24,  25,  127, 127, 127],
    [23,  24,  24,  24,  24,  24,  24,  24,  24,  24,  24,  25,  127],
    [23,  24,  24,  24,  24,  24,  24,  24,  24,  24,  25,  127, 127],
    [23,  24,  24,  24,  24,  24,  24,  24,  24,  24,  24,  24,  25 ],
];

#[rustfmt::skip]
const meters: [[u8; 13]; 14] = [
    [127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [26,  127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [26,  28,  127, 127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [26,  27,  28,  127, 127, 127, 127, 127, 127, 127, 127, 127, 127],
    [26,  27,  27,  28,  127, 127, 127, 127, 127, 127, 127, 127, 127],
    [26,  27,  27,  27,  28,  127, 127, 127, 127, 127, 127, 127, 127],
    [26,  27,  27,  27,  27,  28,  127, 127, 127, 127, 127, 127, 127],
    [26,  27,  27,  27,  27,  27,  28,  127, 127, 127, 127, 127, 127],
    [26,  27,  27,  27,  27,  27,  27,  28,  127, 127, 127, 127, 127],
    [26,  27,  27,  27,  27,  27,  27,  27,  28,  127, 127, 127, 127],
    [26,  27,  27,  27,  27,  27,  27,  27,  27,  28,  127, 127, 127],
    [26,  27,  27,  27,  27,  27,  27,  27,  27,  27,  28,  127, 127],
    [26,  27,  27,  27,  27,  27,  27,  27,  27,  27,  27,  28,  127],
    [26,  27,  27,  27,  27,  27,  27,  27,  27,  27,  27,  27,  28 ],
];

const opposite: [dirtype; 9] = [
    south, west, north, east, southwest, northwest, northeast, southeast, nodir,
];

/*=====================================*/
/*				       */
/* newobject                           */
/* returns the number of a free object */
/*				       */
/*=====================================*/

fn newobject(gs: &mut GlobalState) -> i32 {
    let mut found_i = None;

    for i in 1..=gs.numobj {
        if { gs.o[i as usize].class } == nothing {
            found_i = Some(i);
            break;
        }
    }

    if found_i.is_none() {
        if gs.numobj < maxobj {
            gs.numobj += 1;
        }
        found_i = Some(gs.numobj);
    }

    let found_i = found_i.unwrap();

    gs.o[found_i as usize].oldtile = -1;
    gs.o[found_i as usize].oldx = 0;
    gs.o[found_i as usize].oldy = 0;

    found_i
}

/*=================================*/
/*				   */
/* printscore / printhighscore     */
/* prints the scores to the screen */
/*				   */
/*=================================*/

pub fn printscore(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    pcs.sx = 31;
    pcs.sy = 3;
    print_str(&pcs.score.to_string(), gs, pcs);
}

pub fn printhighscore(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    pcs.sx = 31;
    pcs.sy = 5;
    print_str(&{ pcs.highscores[1].score }.to_string(), gs, pcs);
}

/*======================================*/
/*				        */
/* printshotpower                       */
/* printbody                            */
/* draws the meter to the current value */
/*				        */
/*======================================*/

pub fn printshotpower(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    pcs.sx = 25;
    pcs.sy = 13;
    if gs.shotpower == 13 {
        print(&altmeters[13], gs, pcs);
    } else {
        print(&meters[gs.shotpower as usize], gs, pcs);
    };
}

pub fn printbody(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    pcs.sx = 25;
    pcs.sy = 16;
    if gs.o[0].hp as i32 > 6 {
        print(&meters[gs.o[0].hp as usize], gs, pcs);
    } else {
        print(&altmeters[gs.o[0].hp as usize], gs, pcs);
    };
}

/*=============================*/
/*			       */
/* levelcleared                */
/* goes to the next level, or  */
/* }s game if all levels done  */
/* checks for warp teleporters */
/*			       */
/*=============================*/

// Rust port: this routine seems to have had two bugs - one in the original, and the other in the SDL
// port.
//
// In the original, the [second warp assignment](https://github.com/64kramsystem/catacomb_ii-64k/blob/master/original_project/CAT_PLAY.C#L120)
// has a index of 2, while warp has (originally) a length of 2. It probably should have been 1.
//
// In the port, such assignment has been left to 2, and the warp size has been [increased to 3](https://github.com/64kramsystem/catacomb_ii-64k/blob/master/sdl_port_project/cat_play.c#L106),
// however, the assignment still doesn't make sense.
// Based on the history, the developer increased the array size by 1 because of a Clang warning, but
// didn't realize why the warning was raised.
//
// Interestingly, this a corrupting off-by-one error, and it was not easily detected because the atoi()
// API ignores trailing junk (🤦).
//
fn levelcleared(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut warp: Vec<u8> = vec![0; 2];

    gs.leveldone = true;

    // Rust port: for convenience
    let altobj_y = gs.altobj.y as usize;
    let altobj_x = gs.altobj.x as usize;

    warp[0] = (gs.background[altobj_y + 2][altobj_x] - 161) as u8;

    if (warp[0]) < b'0' || warp[0] > b'9' {
        warp[0] = b'0';
    }

    warp[1] = (gs.background[altobj_y + 2][altobj_x + 1] - 161) as u8;

    if (warp[1]) < b'0' || warp[1] > b'9' {
        warp[1] = b' ';
    }

    let value = String::from_utf8(warp)
        .unwrap()
        .trim()
        .parse::<i16>()
        .unwrap();

    if value > 0 {
        pcs.level = value;
    } else {
        pcs.level += 1;
    }

    if pcs.level > 30 {
        /*all levels have been completed*/
        gs.playdone = true;
        gs.gamexit = victorious;
    }
}

fn givekey(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    i = gs.items[1] as i32 + 1;
    gs.items[1] = i as i16;
    if i < 11 {
        drawchar(26 + i, 7, 31, gs, pcs);
    }
}

pub fn givepotion(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    i = gs.items[2] as i32 + 1;
    gs.items[2] = i as i16;
    if i < 11 {
        drawchar(26 + i, 8, 29, gs, pcs);
    }
}

pub fn givebolt(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    i = gs.items[3] as i32 + 1;
    gs.items[3] = i as i16;
    if i < 11 {
        drawchar(26 + i, 9, 30, gs, pcs);
    }
}

pub fn givenuke(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    i = gs.items[5] as i32 + 1;
    gs.items[5] = i as i16;
    if i < 11 {
        drawchar(26 + i, 10, 30, gs, pcs);
    }
}

fn takekey(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) -> bool {
    let mut i: i32 = 0;
    if gs.items[1] as i32 > 0 {
        i = gs.items[1] as i32 - 1;
        gs.items[1] = i as i16;
        if i < 10 {
            drawchar(27 + i, 7, 32, gs, pcs);
        }
        PlaySound(11, pas);
        true
    } else {
        PlaySound(14, pas);
        false
    }
}

fn takepotion(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    if gs.items[2] as i32 > 0 {
        i = gs.items[2] as i32 - 1;
        gs.items[2] = i as i16;
        if i < 11 {
            drawchar(27 + i, 8, 32, gs, pcs);
        }
        PlaySound(12, pas);
        gs.o[0].hp = 13;
        gs.obj.hp = 13;
        printbody(gs, pcs);
    } else {
        PlaySound(14, pas);
    };
}

fn castbolt(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    if gs.items[3] as i32 > 0 {
        i = gs.items[3] as i32 - 1;
        gs.items[3] = i as i16;
        if i < 11 {
            drawchar(27 + i, 9, 32, gs, pcs);
        }
        gs.boltsleft = 8;
        PlaySound(13, pas);
    } else {
        PlaySound(14, pas);
    };
}

fn castnuke(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut n: i32 = 0;
    let mut base: activeobj = activeobj {
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
    };
    if gs.items[5] as i32 == 0 {
        PlaySound(14, pas);
        return;
    }
    i = gs.items[5] as i32 - 1;
    gs.items[5] = i as i16;
    if i < 11 {
        drawchar(27 + i, 10, 32, gs, pcs);
    }
    base.delay = 0;
    base.stage = 0;
    base.active = true;
    base.x = gs.obj.x;
    base.y = gs.obj.y;
    base.oldx = base.x;
    base.oldy = base.y;
    base.oldtile = -1_i16;
    base.class = bigshot;
    x = -1;
    while x <= 1 {
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].x = (gs.o[n as usize].x as i32 + x * 2) as u8;
        gs.o[n as usize].dir = north as i32 as u16;
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].x = (gs.o[n as usize].x as i32 + x * 2) as u8;
        gs.o[n as usize].dir = south as i32 as u16;
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].y = (gs.o[n as usize].y as i32 + x * 2) as u8;
        gs.o[n as usize].dir = east as i32 as u16;
        n = newobject(gs);
        gs.o[n as usize] = base;
        gs.o[n as usize].y = (gs.o[n as usize].y as i32 + x * 2) as u8;
        gs.o[n as usize].dir = west as i32 as u16;
        x += 1;
    }
    PlaySound(13, pas);
    gs.obj.stage = 2;
    gs.obj.delay = 4;
}

fn playshoot(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    let mut new: i32 = 0;
    gs.obj.stage = 2;
    gs.obj.delay = 4;
    PlaySound(5, pas);
    new = newobject(gs);
    gs.o[new as usize].class = shot;
    gs.side ^= 1;
    gs.o[new as usize].delay = 0;
    gs.o[new as usize].stage = 0;
    gs.o[new as usize].active = true;
    gs.o[new as usize].dir = gs.obj.dir;
    match gs.o[new as usize].dir as i32 {
        0 => {
            gs.o[new as usize].x = (gs.obj.x as i32 + gs.side) as u8;
            gs.o[new as usize].y = gs.obj.y;
        }
        1 => {
            gs.o[new as usize].x = (gs.obj.x as i32 + 1) as u8;
            gs.o[new as usize].y = (gs.obj.y as i32 + gs.side) as u8;
        }
        2 => {
            gs.o[new as usize].x = (gs.obj.x as i32 + gs.side) as u8;
            gs.o[new as usize].y = (gs.obj.y as i32 + 1) as u8;
        }
        3 => {
            gs.o[new as usize].x = gs.obj.x;
            gs.o[new as usize].y = (gs.obj.y as i32 + gs.side) as u8;
        }
        _ => {}
    };
}

fn playbigshoot(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    let mut new: i32 = 0;
    gs.obj.stage = 2;
    if gs.boltsleft == 0 {
        gs.obj.delay = 4;
    }
    PlaySound(4, pas);
    new = newobject(gs);
    gs.o[new as usize].delay = 0;
    gs.o[new as usize].stage = 0;
    gs.o[new as usize].active = true;
    gs.o[new as usize].dir = gs.obj.dir;
    gs.o[new as usize].x = gs.obj.x;
    gs.o[new as usize].y = gs.obj.y;
    gs.o[new as usize].class = bigshot;
}

fn givescroll(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    if rndt(pas) < 128 {
        givebolt(gs, pcs);
    } else {
        givenuke(gs, pcs);
    };
}

fn opendoor(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    PlaySound(11, pas);
    x = gs.chkx;
    y = gs.chky;
    if gs.chkspot == 165 {
        loop {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            y -= 1;
            if !(gs.view[y as usize][x as usize] == 165) {
                break;
            }
        }
        y = gs.chky + 1;
        while gs.view[y as usize][x as usize] == 165 {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            y += 1;
        }
    } else {
        loop {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            x -= 1;
            if !(gs.view[y as usize][x as usize] == 166) {
                break;
            }
        }
        x = gs.chkx + 1;
        while gs.view[y as usize][x as usize] == 166 {
            gs.view[y as usize][x as usize] = 128;
            gs.background[y as usize][x as usize] = 128;
            x += 1;
        }
    };
}

fn tagobject(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let i: i32 = gs.altobj.hp as i32;
    if gs.GODMODE && gs.altobj.class as i32 == player as i32 {
        return;
    }
    gs.altobj.hp = (gs.altobj.hp as i32 - gs.obj.damage as i32) as i8;
    if i <= gs.obj.damage as i32 {
        if gs.altobj.class as i32 == player as i32 {
            gs.o[0].hp = 0;
            gs.altobj.hp = gs.o[0].hp;
            printbody(gs, pcs);
            PlaySound(10, pas);
            gs.playdone = true;
            gs.gamexit = killed;
        } else {
            pcs.score += gs.altobj.points as i32;
            printscore(gs, pcs);
            PlaySound(9, pas);
        }
        gs.o[gs.altnum as usize].class = (dead1 as u16 - 1 + gs.altobj.size as u16).into();
        gs.o[gs.altnum as usize].delay = 2;
        gs.o[gs.altnum as usize].stage = 0;
    } else {
        if gs.o[gs.altnum as usize].class as i32 == guns as i32
            || gs.o[gs.altnum as usize].class as i32 == gune as i32
        {
            return;
        }
        gs.o[gs.altnum as usize].hp = gs.altobj.hp;
        gs.o[gs.altnum as usize].stage = 3;
        if gs.altnum == 0 {
            gs.o[0].delay = 2;
            printbody(gs, pcs);
            PlaySound(8, pas);
        } else {
            gs.o[gs.altnum as usize].delay = 4;
            PlaySound(7, pas);
        }
    };
}

/*==============================*/
/*			        */
/* intomonster                  */
/* obj contacted another object */
/*			        */
/*==============================*/

fn intomonster(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) -> bool {
    let mut gotit = false;

    /*figure out which object got hit*/

    gs.altnum = 0;
    gotit = false;
    loop {
        /* make a copy of the objects info into a global varriable */

        gs.altobj.update_from_activeobj(&gs.o[gs.altnum as usize]);
        if gs.altobj.class as i32 > nothing as i32 && gs.altnum != gs.objecton {
            gs.altobj
                .update_from_objdeftype(&gs.objdef[gs.altobj.class as usize]);

            if gs.chkx >= gs.altobj.x as i32
                && (gs.chkx - gs.altobj.x as i32) < gs.altobj.size as i32
                && gs.chky >= gs.altobj.y as i32
                && (gs.chky - gs.altobj.y as i32) < gs.altobj.size as i32
            {
                if gs.altobj.solid != 0 {
                    gotit = true;
                } else if gs.objecton == 0
                    && (gs.altobj.class as i32 == teleporter as i32
                        || gs.altobj.class as i32 == secretgate as i32)
                {
                    /*player got to the teleporter*/
                    levelcleared(gs, pcs);
                }
            }
        }
        if !gotit {
            gs.altnum += 1;
        }
        if gotit as i32 != 0 || gs.altnum > gs.numobj {
            break;
        }
    }
    if !gotit {
        return true;
    }

    /*resolve contact based on attacker and target*/

    match gs.obj.contact as i32 {
        0 => return false, /*benign objects just don't move through others*/
        1 | 3 => {
            if gs.altnum == 0 {
                tagobject(gs, pas, pcs);
                gs.obj.stage = 2; /*set it to attack stage*/
                gs.obj.delay = 20; /*delay for several frames*/
            } else if gs.altobj.class as i32 == shot as i32 {
                /*they can walk into shots*/
                return true;
            }
            return false;
        }
        2 => {
            if gs.altnum > 0 {
                tagobject(gs, pas, pcs);
            }
            return false;
        }
        4 => {
            tagobject(gs, pas, pcs);
            return true; /*nuke shots keep going*/
        }
        _ => {}
    }
    false
}

fn walkthrough(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) -> bool {
    let mut new: i32 = 0;
    if gs.chkspot == 128 {
        return true;
    }
    if gs.chkspot >= 256 && gs.chkspot <= 256 + 67 * 4 + 35 * 9 + 19 * 16 + 19 * 25 {
        return intomonster(gs, pas, pcs);
    }
    if gs.chkspot >= 129 && gs.chkspot <= 135 {
        if gs.obj.contact as i32 == pshot as i32
            || gs.obj.contact as i32 == nukeshot as i32
            || gs.obj.contact as i32 == mshot as i32
        {
            new = newobject(gs);
            gs.o[new as usize].active = true;
            gs.o[new as usize].x = gs.chkx as u8;
            gs.o[new as usize].y = gs.chky as u8;
            gs.o[new as usize].stage = 0;
            gs.o[new as usize].delay = 2;
            gs.o[new as usize].class = wallhit;
            PlaySound(6, pas);
        }
        return false;
    }
    if gs.chkspot >= 136 && gs.chkspot <= 145 {
        if gs.obj.contact as i32 == pshot as i32 || gs.obj.contact as i32 == nukeshot as i32 {
            PlaySound(6, pas);
            if gs.chkspot < 143 {
                gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            } else {
                gs.background[gs.chky as usize][gs.chkx as usize] = gs.chkspot + 19;
            }
            new = newobject(gs);
            gs.o[new as usize].active = true;
            gs.o[new as usize].x = gs.chkx as u8;
            gs.o[new as usize].y = gs.chky as u8;
            gs.o[new as usize].stage = 0;
            gs.o[new as usize].delay = 2;
            gs.o[new as usize].class = dead1;
            return gs.obj.contact != pshot as u8;
        } else {
            return false;
        }
    }
    if gs.chkspot == 162 {
        if gs.obj.class as i32 == player as i32 {
            givepotion(gs, pcs);
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(2, pas);
        }
        return true;
    }
    if gs.chkspot == 163 {
        if gs.obj.class as i32 == player as i32 {
            givescroll(gs, pas, pcs);
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(2, pas);
        }
        return true;
    }
    if gs.chkspot == 164 {
        if gs.obj.class as i32 == player as i32 {
            givekey(gs, pcs);
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(2, pas);
        }
        return true;
    }
    if gs.chkspot == 165 || gs.chkspot == 166 {
        if gs.obj.class as i32 == player as i32 {
            if takekey(gs, pas, pcs) {
                opendoor(gs, pas);
                return true;
            }
        }
        return false;
    }
    if gs.chkspot == 167 {
        if gs.obj.class as i32 == player as i32 {
            pcs.score += 500;
            printscore(gs, pcs);
            gs.background[gs.chky as usize][gs.chkx as usize] = 128;
            gs.view[gs.chky as usize][gs.chkx as usize] = 128;
            PlaySound(3, pas);
        }
        return true;
    }
    if gs.chkspot >= 29 && gs.chkspot <= 31 {
        return true;
    }
    false
}

fn walk(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) -> bool {
    let mut i: i32 = 0;
    let mut newx: i32 = 0;
    let mut newy: i32 = 0;
    let mut deltay: i32 = 0;
    let mut deltax: i32 = 0;
    let mut try_0 = false;
    match gs.obj.dir as i32 {
        0 => {
            newx = gs.obj.x as i32;
            newy = gs.obj.y as i32 - 1;
            gs.chkx = newx;
            gs.chky = newy;
            deltax = 1;
            deltay = 0;
        }
        1 => {
            newx = gs.obj.x as i32 + 1;
            newy = gs.obj.y as i32;
            gs.chkx = gs.obj.x as i32 + gs.obj.size as i32;
            gs.chky = newy;
            deltax = 0;
            deltay = 1;
        }
        2 => {
            newx = gs.obj.x as i32;
            newy = gs.obj.y as i32 + 1;
            gs.chkx = newx;
            gs.chky = gs.obj.y as i32 + gs.obj.size as i32;
            deltax = 1;
            deltay = 0;
        }
        3 => {
            newx = gs.obj.x as i32 - 1;
            newy = gs.obj.y as i32;
            gs.chkx = newx;
            gs.chky = newy;
            deltax = 0;
            deltay = 1;
        }
        _ => return false,
    }
    i = 1;
    while i <= gs.obj.size as i32 {
        gs.chkspot = gs.view[gs.chky as usize][gs.chkx as usize];
        if gs.chkspot != 128 {
            try_0 = walkthrough(gs, pas, pcs);
            if gs.leveldone {
                return true;
            }
            if gs.obj.stage as i32 == 2 {
                return true;
            }
            if !try_0 {
                return false;
            }
        }
        gs.chkx += deltax;
        gs.chky += deltay;
        i += 1;
    }
    gs.obj.x = newx as u8;
    gs.obj.y = newy as u8;
    gs.obj.stage = (gs.obj.stage as i32 ^ 1) as u8;
    true
}

fn playercmdthink(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    let mut olddir: dirtype = north;
    let mut c: ControlStruct = ControlStruct {
        dir: north,
        button1: false,
        button2: false,
    };
    c = ControlPlayer(1, gs, pcs, pas, sdl);
    gs.obj.stage = (gs.obj.stage as i32 & 1) as u8;
    if c.button1 as i32 != 0
        && c.button2 as i32 != 0
        && pcs.keydown[SDL_SCANCODE_Q as usize] as i32 != 0
    {
        givepotion(gs, pcs);
        givescroll(gs, pas, pcs);
        givekey(gs, pcs);
    }
    if (c.dir as u32) < nodir as i32 as u32 && gs.frameon as i32 % 2 != 0 {
        if c.button2 {
            olddir = gs.obj.dir.into();
        }
        if c.dir as u32 > west as i32 as u32 {
            if gs.frameon as i32 / 2 % 2 != 0 {
                match c.dir as u32 {
                    4 => {
                        gs.obj.dir = east as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = north;
                    }
                    5 => {
                        gs.obj.dir = south as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = east;
                    }
                    6 => {
                        gs.obj.dir = west as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = south;
                    }
                    7 => {
                        gs.obj.dir = north as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = west;
                    }
                    _ => {}
                }
            } else {
                match c.dir as u32 {
                    4 => {
                        gs.obj.dir = north as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = east;
                    }
                    5 => {
                        gs.obj.dir = east as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = south;
                    }
                    6 => {
                        gs.obj.dir = south as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = west;
                    }
                    7 => {
                        gs.obj.dir = west as i32 as u16;
                        walk(gs, pas, pcs);
                        c.dir = north;
                    }
                    _ => {}
                }
            }
        }
        gs.obj.dir = c.dir as u16;
        if !walk(gs, pas, pcs) {
            PlaySound(1, pas);
        }
        if c.button2 {
            gs.obj.dir = olddir as u16;
        }
    } else if !c.button2 {
        match c.dir as u32 {
            7 | 0 => {
                gs.obj.dir = north as i32 as u16;
            }
            4 | 1 => {
                gs.obj.dir = east as i32 as u16;
            }
            5 | 2 => {
                gs.obj.dir = south as i32 as u16;
            }
            6 | 3 => {
                gs.obj.dir = west as i32 as u16;
            }
            8 | _ => {}
        }
    }
    gs.origin.x = gs.obj.x as i32 - 11;
    gs.origin.y = gs.obj.y as i32 - 11;
    if gs.boltsleft > 0 {
        if gs.frameon as i32 % 3 == 0 {
            playbigshoot(gs, pas);
            gs.boltsleft -= 1;
        }
    } else if c.button1 {
        if gs.shotpower == 0 {
            gs.shotpower = 1;
        } else if gs.shotpower < 13 && gs.frameon as i32 % 2 != 0 {
            gs.shotpower += 1;
        }
        printshotpower(gs, pcs);
    } else if gs.shotpower > 0 {
        if gs.shotpower == 13 {
            playbigshoot(gs, pas);
        } else {
            playshoot(gs, pas);
        }
        gs.shotpower = 0;
        printshotpower(gs, pcs);
    }
    if gs.indemo == notdemo {
        if pcs.keydown[SDL_SCANCODE_P as usize] as i32 != 0
            || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            if (gs.obj.hp as i32) < 13 {
                takepotion(gs, pas, pcs);
                pcs.keydown[SDL_SCANCODE_Q as usize] = false;
                pcs.keydown[SDL_SCANCODE_SPACE as usize] = false;
            }
        } else if pcs.keydown[SDL_SCANCODE_B as usize] {
            castbolt(gs, pas, pcs);
            pcs.keydown[SDL_SCANCODE_B as usize] = false;
        } else if pcs.keydown[SDL_SCANCODE_N as usize] as i32 != 0
            || pcs.keydown[SDL_SCANCODE_RETURN as usize] as i32 != 0
        {
            castnuke(gs, pas, pcs);
            pcs.keydown[SDL_SCANCODE_N as usize] = false;
            pcs.keydown[SDL_SCANCODE_RETURN as usize] = false;
        }
    }
    dofkeys(gs, cps, pas, pcs, sdl);
    if gs.resetgame {
        gs.resetgame = false;
        gs.playdone = true;
        return;
    }
    match gs.indemo {
        notdemo => {
            if pcs.keydown[SDL_SCANCODE_C as usize] as i32 != 0
                && pcs.keydown[SDL_SCANCODE_T as usize] as i32 != 0
                && pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                centerwindow(16, 2, gs, pcs);
                print_str("warp to which\nlevel (1-99)?", gs, pcs);
                clearkeys(pcs, pas, sdl);
                pcs.level = _inputint(gs, pcs, pas, sdl) as i16;
                if (pcs.level as i32) < 1 {
                    pcs.level = 1;
                }
                if pcs.level as i32 > 30 {
                    pcs.level = 30;
                }
                restore(gs, pcs);
                gs.leveldone = true;
            }
            if pcs.keydown[SDL_SCANCODE_C as usize] as i32 != 0
                && pcs.keydown[SDL_SCANCODE_T as usize] as i32 != 0
                && pcs.keydown[SDL_SCANCODE_TAB as usize] as i32 != 0
            {
                if gs.GODMODE {
                    centerwindow(13, 1, gs, pcs);
                    print_str("God Mode Off", gs, pcs);
                    gs.GODMODE = false;
                } else {
                    centerwindow(12, 1, gs, pcs);
                    print_str("God Mode On", gs, pcs);
                    gs.GODMODE = true;
                }
                UpdateScreen(gs, pcs);
                clearkeys(pcs, pas, sdl);
                while bioskey(0, pcs, pas, sdl) == 0 {
                    WaitVBL();
                }
                restore(gs, pcs);
                clearkeys(pcs, pas, sdl);
            }
        }
        demoplay => {
            gs.indemo = notdemo;
            gs.ctrl = ControlPlayer(1, gs, pcs, pas, sdl);
            if gs.ctrl.button1 as i32 != 0
                || gs.ctrl.button2 as i32 != 0
                || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                gs.indemo = demoplay;
                gs.exitdemo = true;
                gs.leveldone = true;
                pcs.level = 0;
                return;
            }
            gs.indemo = demoplay;
        }
        _ => {}
    };
}

fn chasethink(
    diagonal: bool,
    gs: &mut GlobalState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
) {
    let mut deltax: i32 = 0;
    let mut deltay: i32 = 0;
    let mut d: [dirtype; 3] = [north; 3];
    let mut tdir: i32 = 0;
    let mut olddir: i32 = 0;
    let mut turnaround: i32 = 0;
    gs.obj.stage = (gs.obj.stage as i32 & 1) as u8;
    olddir = gs.obj.dir as i32;
    turnaround = opposite[olddir as usize] as i32;
    deltax = gs.o[0].x as i32 - gs.obj.x as i32;
    deltay = gs.o[0].y as i32 - gs.obj.y as i32;
    d[1] = nodir;
    d[2] = nodir;
    if deltax > 0 {
        d[1] = east;
    }
    if deltax < 0 {
        d[1] = west;
    }
    if deltay > 0 {
        d[2] = south;
    }
    if deltay < 0 {
        d[2] = north;
    }
    if deltay.abs() > deltax.abs() {
        tdir = d[1] as i32;
        d[1] = d[2];
        d[2] = tdir.into();
    }
    if d[1] as u32 == turnaround as u32 {
        d[1] = nodir;
    }
    if d[2] as u32 == turnaround as u32 {
        d[2] = nodir;
    }
    if diagonal {
        if d[1] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[1] as u16;
            if walk(gs, pas, pcs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
        if d[2] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[2] as u16;
            if walk(gs, pas, pcs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
    } else {
        if d[2] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[2] as u16;
            if walk(gs, pas, pcs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
        if d[1] as u32 != nodir as i32 as u32 {
            gs.obj.dir = d[1] as u16;
            if walk(gs, pas, pcs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                return;
            }
        }
    }
    gs.obj.dir = olddir as u16;
    if walk(gs, pas, pcs) as i32 != 0 || gs.obj.stage as i32 == 3 {
        return;
    }
    if rndt(pas) > 128 {
        tdir = north as i32;
        while tdir <= west as i32 {
            if tdir != turnaround {
                gs.obj.dir = tdir as u16;
                if walk(gs, pas, pcs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                    return;
                }
            }
            tdir += 1;
        }
    } else {
        tdir = west as i32;
        while tdir >= north as i32 {
            if tdir != turnaround {
                gs.obj.dir = tdir as u16;
                if walk(gs, pas, pcs) as i32 != 0 || gs.obj.stage as i32 == 3 {
                    return;
                }
            }
            tdir -= 1;
        }
    }
    gs.obj.dir = turnaround as u16;
    walk(gs, pas, pcs);
}

fn gargthink(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut n: i32 = 0;
    if rndt(pas) > 220 {
        gs.obj.stage = 2;
        gs.obj.delay = 6;
        PlaySound(5, pas);
        n = newobject(gs);
        gs.o[n as usize].class = rock;
        gs.o[n as usize].delay = 0;
        gs.o[n as usize].stage = 0;
        gs.o[n as usize].active = true;
        gs.o[n as usize].dir = gs.obj.dir;
        match gs.obj.dir as i32 {
            0 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = gs.obj.y;
            }
            1 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 3) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            2 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 3) as u8;
            }
            3 => {
                gs.o[n as usize].x = gs.obj.x;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            _ => {}
        }
    } else {
        chasethink(false, gs, pas, pcs);
    };
}

fn dragonthink(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut n: i32 = 0;
    if rndt(pas) > 220 {
        gs.obj.stage = 2;
        gs.obj.delay = 6;
        PlaySound(5, pas);
        n = newobject(gs);
        gs.o[n as usize].class = bigshot;
        gs.o[n as usize].delay = 0;
        gs.o[n as usize].stage = 0;
        gs.o[n as usize].active = true;
        gs.o[n as usize].dir = gs.obj.dir;
        match gs.o[n as usize].dir as i32 {
            0 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = gs.obj.y;
            }
            1 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 3) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            2 => {
                gs.o[n as usize].x = (gs.obj.x as i32 + 1 + gs.side) as u8;
                gs.o[n as usize].y = (gs.obj.y as i32 + 3) as u8;
            }
            3 => {
                gs.o[n as usize].x = gs.obj.x;
                gs.o[n as usize].y = (gs.obj.y as i32 + 1 + gs.side) as u8;
            }
            _ => {}
        }
    } else {
        chasethink(false, gs, pas, pcs);
    };
}

fn gunthink(dir: i32, gs: &mut GlobalState, pas: &mut PcrlibAState) {
    let mut n: i32 = 0;
    PlaySound(5, pas);
    gs.obj.stage = 0;
    n = newobject(gs);
    gs.o[n as usize].class = bigshot;
    gs.o[n as usize].delay = 0;
    gs.o[n as usize].stage = 0;
    gs.o[n as usize].active = true;
    gs.o[n as usize].dir = dir as u16;
    gs.o[n as usize].x = gs.obj.x;
    gs.o[n as usize].y = gs.obj.y;
}

fn shooterthink(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    if (gs.obj.x as i32) < gs.origin.x - 1
        || (gs.obj.y as i32) < gs.origin.y - 1
        || gs.obj.x as i32 > gs.origin.x + 22
        || gs.obj.y as i32 > gs.origin.y + 22
        || !walk(gs, pas, pcs)
        || gs.obj.stage as i32 == 2
    {
        gs.obj.class = nothing;
    }
}

fn idlethink(gs: &mut GlobalState) {
    gs.obj.stage = (gs.obj.stage).wrapping_add(1);
    gs.obj.delay = 2;
    if gs.obj.stage as i32 == gs.obj.stages as i32 {
        gs.obj.stage = 0;
    }
}

fn fadethink(gs: &mut GlobalState) {
    gs.obj.stage = (gs.obj.stage).wrapping_add(1);
    gs.obj.delay = 2;
    if gs.obj.stage as i32 == gs.obj.stages as i32 {
        gs.obj.class = nothing;
    }
}

fn killnear(chkx_0: i32, chky_0: i32, gs: &mut GlobalState, pas: &mut PcrlibAState) {
    let mut spot: i32 = 0;
    let mut new: i32 = 0;
    spot = gs.background[chky_0 as usize][chkx_0 as usize];
    if spot < 136 || spot > 145 {
        return;
    }
    PlaySound(6, pas);
    if spot < 143 {
        gs.background[chky_0 as usize][chkx_0 as usize] = 128;
    } else {
        gs.background[chky_0 as usize][chkx_0 as usize] = spot + 19;
    }
    new = newobject(gs);
    gs.o[new as usize].active = true;
    gs.o[new as usize].x = chkx_0 as u8;
    gs.o[new as usize].y = chky_0 as u8;
    gs.o[new as usize].stage = 0;
    gs.o[new as usize].delay = 2;
    gs.o[new as usize].class = dead1;
}

fn explodethink(gs: &mut GlobalState, pas: &mut PcrlibAState) {
    gs.obj.stage = (gs.obj.stage).wrapping_add(1);
    if gs.obj.stage as i32 == 1 {
        killnear(gs.obj.x as i32 - 1, gs.obj.y as i32, gs, pas);
        killnear(gs.obj.x as i32, gs.obj.y as i32 - 1, gs, pas);
        killnear(gs.obj.x as i32 + 1, gs.obj.y as i32, gs, pas);
        killnear(gs.obj.x as i32, gs.obj.y as i32 + 1, gs, pas);
    }
    gs.obj.delay = 2;
    if gs.obj.stage as i32 == gs.obj.stages as i32 {
        gs.obj.class = nothing;
    }
}

fn think(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    if gs.obj.delay as i32 > 0 {
        gs.obj.delay = (gs.obj.delay).wrapping_sub(1);
    } else if rndt(pas) < gs.obj.speed as i32 {
        match gs.obj.think as i32 {
            0 => {
                playercmdthink(gs, cps, pas, pcs, sdl);
            }
            3 => {
                chasethink(false, gs, pas, pcs);
            }
            4 => {
                chasethink(true, gs, pas, pcs);
            }
            1 => {
                gargthink(gs, pas, pcs);
            }
            2 => {
                dragonthink(gs, pas, pcs);
            }
            5 => {
                shooterthink(gs, pas, pcs);
            }
            6 => {
                idlethink(gs);
            }
            7 => {
                fadethink(gs);
            }
            8 => {
                explodethink(gs, pas);
            }
            9 => {
                gunthink(west as i32, gs, pas);
            }
            10 => {
                gunthink(north as i32, gs, pas);
            }
            _ => {}
        }
    }
}

pub fn doactive(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    if gs.obj.class as i32 != dead1 as i32
        && ((gs.obj.x as i32) < gs.origin.x - 10
            || gs.obj.x as i32 > gs.origin.x + 34
            || (gs.obj.y as i32) < gs.origin.y - 10
            || gs.obj.y as i32 > gs.origin.y + 34)
    {
        gs.o[gs.objecton as usize].active = false;
    } else {
        think(gs, cps, pas, pcs, sdl);
        eraseobj(gs);
        if gs.playdone {
            return;
        }
        if gs.obj.class as i32 > nothing as i32 {
            drawobj(gs);
        }
        gs.o[gs.objecton as usize] = gs.obj.into();
    };
}

pub fn doinactive(gs: &mut GlobalState) {
    if gs.obj.x as i32 + gs.obj.size as i32 >= gs.origin.x
        && (gs.obj.x as i32) < gs.origin.x + 24
        && gs.obj.y as i32 + gs.obj.size as i32 >= gs.origin.y
        && (gs.obj.y as i32) < gs.origin.y + 24
    {
        gs.obj.active = true;
        gs.obj.dir = north as i32 as u16;
        gs.o[gs.objecton as usize] = gs.obj.into();
    }
}

pub fn playloop(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    gs.screencenter.x = 11;
    loop {
        if gs.indemo == notdemo {
            centerwindow(11, 2, gs, pcs);
            print_str(" Entering\nlevel ", gs, pcs);
            print_str(&pcs.level.to_string(), gs, pcs);
            print_str("...", gs, pcs);
            PlaySound(17, pas);
            WaitEndSound(gs, pas, pcs);
        }
        clearold(&mut gs.oldtiles);
        loadlevel(gs, pas, pcs);
        gs.leveldone = false;
        if pcs.keydown[SDL_SCANCODE_F7 as usize] as i32 != 0
            && pcs.keydown[SDL_SCANCODE_D as usize] as i32 != 0
        {
            clearold(&mut gs.oldtiles);
            refresh(gs, pcs);
            refresh(gs, pcs);
            clearkeys(pcs, pas, sdl);
            centerwindow(12, 1, gs, pcs);
            print_str("RECORD DEMO", gs, pcs);
            loop {
                let ch = get(gs, pcs, pas, sdl) as i8;
                if !(ch != 13) {
                    break;
                }
            }
            RecordDemo(gs, pcs);
            clearold(&mut gs.oldtiles);
            clearkeys(pcs, pas, sdl);
        }
        gs.playdone = false;
        gs.frameon = 0;
        gs.boltsleft = 0;
        gs.shotpower = 0;
        initrndt(false, pas);
        printshotpower(gs, pcs);
        doall(gs, cps, pas, pcs, sdl);
        if gs.indemo == recording {
            clearkeys(pcs, pas, sdl);
            centerwindow(15, 1, gs, pcs);
            print_str("SAVE AS DEMO#:", gs, pcs);
            let mut ch;
            loop {
                ch = get(gs, pcs, pas, sdl) as i8;
                if !(ch < '0' as i8 || ch > '9' as i8) {
                    break;
                }
            }
            SaveDemo((ch - '0' as i8) as u8, gs, pcs);
            clearold(&mut gs.oldtiles);
            refresh(gs, pcs);
            refresh(gs, pcs);
        }
        if gs.indemo != notdemo {
            gs.playdone = true;
        }
        if gs.playdone {
            break;
        }
    }
}
