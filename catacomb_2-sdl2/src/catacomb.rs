use std::fs::File;

use serdine::{Deserialize, Serialize};

use crate::{
    cat_play::{
        givebolt, givenuke, givepotion, playloop, printbody, printhighscore, printscore,
        printshotpower,
    },
    catasm::{cgarefresh, drawchartile, egarefresh},
    class_type::classtype::{self, *},
    cpanel::{controlpanel, installgrfile},
    cpanel_state::CpanelState,
    demo_enum::demoenum::*,
    dir_type::dirtype::{self, *},
    exit_type::exittype::*,
    extra_constants::{
        blankfloor, leftoff, maxpics, numlevels, solidwall, tile2s, topoff, NUM_DEMOS,
    },
    global_state::GlobalState,
    gr_type::grtype::*,
    objects::initobjects,
    pcrlib_a::{drawchar, drawpic, rnd, rndt, PlaySound, WaitEndSound, WaitVBL},
    pcrlib_a_state::PcrlibAState,
    pcrlib_c::{
        ControlPlayer, LoadDemo, UpdateScreen, _Verify, _checkhighscore, _quit, _setupgame,
        _showhighscores, bar, bioskey, bloadin, centerwindow, clearkeys, drawwindow, expwin, get,
        loadFile, print_str, printchartile,
    },
    pcrlib_c_state::PcrlibCState,
    rleasm::RLEExpand,
    scan_codes::*,
    sdl_manager::SdlManager,
    state_type::statetype,
};

/*==============================*/
/*			        */
/* xxxrefresh                   */
/* refresh the changed areas of */
/* the tiles map in the various */
/* graphics modes.              */
/*			        */
/*==============================*/

const demowin: [[u8; 16]; 5] = [
    [
        14, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 16,
    ],
    [
        17, b' ', b'-', b'-', b'-', b' ', b'D', b'E', b'M', b'O', b' ', b'-', b'-', b'-', b' ', 18,
    ],
    [
        17, b'S', b'P', b'A', b'C', b'E', b' ', b'T', b'O', b' ', b'S', b'T', b'A', b'R', b'T', 18,
    ],
    [
        17, b'F', b'1', b' ', b'T', b'O', b' ', b'G', b'E', b'T', b' ', b'H', b'E', b'L', b'P', 18,
    ],
    [
        19, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 21,
    ],
];

pub fn refresh(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut underwin = [[0; 16]; 5];

    let basex = gs.origin.x as usize + 4;
    let basey = gs.origin.y as usize + 17;
    if gs.indemo != notdemo {
        for y in 0..=4 {
            for x in 0..=15 {
                underwin[y][x] = gs.view[(y + basey)][(x + basex)] as u16;
                gs.view[(y + basey)][(x + basex)] = demowin[y][x] as i32;
            }
        }
    }

    WaitVBL();
    if pcs.grmode == CGAgr {
        cgarefresh(gs, pcs);
    } else {
        egarefresh(gs, pcs);
    }

    if gs.indemo != notdemo {
        // Using an iterator makes this less readable.
        #[allow(clippy::needless_range_loop)]
        for y in 0..=4 {
            for x in 0..=15 {
                gs.view[y + basey][x + basex] = underwin[y][x] as i32;
            }
        }
    }

    WaitVBL();
}

fn simplerefresh(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    WaitVBL();
    if pcs.grmode as u32 == CGAgr as i32 as u32 {
        cgarefresh(gs, pcs);
    } else {
        egarefresh(gs, pcs);
    };
}

/*
===================
=
= loadgrfiles
=
= Loads the tiles and sprites, and sets up the pointers and tables
=
===================
*/

pub fn loadgrfiles(gs: &mut GlobalState, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    if pcs.grmode as u32 == CGAgr as i32 as u32 {
        gs.pics = bloadin("CGACHARS.CA2").unwrap();
        installgrfile("CGAPICS.CA2", cps, pcs);
    } else {
        gs.pics = bloadin("EGACHARS.CA2").unwrap();
        installgrfile("EGAPICS.CA2", cps, pcs);
    };
}

/*======================================*/
/*				        */
/* restore                              */
/* redraws every tile on the tiled area */
/* by setting oldtiles to -1.  used to  */
/* erase any temporary windows.         */
/*				        */
/*======================================*/

pub fn clearold(oldtiles: &mut [i32; 576]) {
    oldtiles.fill(0xff); /*clear all oldtiles*/
}

pub fn restore(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    clearold(&mut gs.oldtiles);
    simplerefresh(gs, pcs);
}

fn wantmore(
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
    pas: &mut PcrlibAState,
    sdl: &mut SdlManager,
) -> bool {
    pcs.sx = 2;
    pcs.sy = 20;
    print_str("(space for more/esc)", gs, pcs);
    pcs.sx = 12;
    pcs.sy = 21;
    let ch = get(gs, pcs, pas, sdl) as i8;
    if ch == 27 {
        return false;
    }
    true
}

fn charpic(
    x: i32,
    y: i32,
    c: classtype,
    dir: dirtype,
    stage: i32,
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
) {
    let mut xx: i32 = 0;
    let mut yy: i32 = 0;
    let mut size: i32 = 0;
    let mut tilenum: i32 = 0;
    size = gs.objdef[c as usize].size as i32;
    tilenum = (gs.objdef[c as usize].firstchar as u32).wrapping_add(
        ((size * size) as u32).wrapping_mul(
            (dir as u32 & gs.objdef[c as usize].dirmask as u32)
                .wrapping_mul(gs.objdef[c as usize].stages as u32)
                .wrapping_add(stage as u32),
        ),
    ) as i32;
    yy = y;
    while yy <= y + size - 1 {
        xx = x;
        while xx <= x + size - 1 {
            let fresh0 = tilenum;
            tilenum += 1;
            drawchartile(xx, yy, fresh0, gs, pcs);
            xx += 1;
        }
        yy += 1;
    }
}

fn help(
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
    pas: &mut PcrlibAState,
    sdl: &mut SdlManager,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    centerwindow(20, 20, gs, pcs);
    print_str("  C A T A C O M B   \n", gs, pcs);
    print_str("   - - - - - - -    \n", gs, pcs);
    print_str(" by John Carmack    \n", gs, pcs);
    print_str("                    \n", gs, pcs);
    print_str("\n", gs, pcs);
    print_str("f1 = help           \n", gs, pcs);
    print_str("f2 = control panel  \n", gs, pcs);
    print_str("f3 = game reset     \n", gs, pcs);
    print_str("f4 = save game      \n", gs, pcs);
    print_str("f5 = load saved game\n", gs, pcs);
    print_str("f9 = pause          \n", gs, pcs);
    print_str("f10 / ESC = quit    \n", gs, pcs);
    print_str("\n", gs, pcs);
    print_str("hit fire at the demo\n", gs, pcs);
    print_str("to begin playing.   \n", gs, pcs);
    if !wantmore(gs, pcs, pas, sdl) {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    print_str("\nKeyboard controls:  \n\n", gs, pcs);
    print_str("move    : arrows    \n", gs, pcs);
    print_str("button1 : ctrl      \n", gs, pcs);
    print_str("button2 : alt       \n", gs, pcs);
    print_str("\nTo switch to mouse \n", gs, pcs);
    print_str("or joystick control,\n", gs, pcs);
    print_str("hit f2             \n", gs, pcs);
    if !wantmore(gs, pcs, pas, sdl) {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    print_str("Button 1 / ctrl key:\n", gs, pcs);
    print_str("Builds shot power.  \n", gs, pcs);
    print_str("If the shot power   \n", gs, pcs);
    print_str("meter is full when  \n", gs, pcs);
    print_str("the button is       \n", gs, pcs);
    print_str("released, a super   \n", gs, pcs);
    print_str("shot will be        \n", gs, pcs);
    print_str("launched.           \n", gs, pcs);
    print_str("\n", gs, pcs);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            drawchartile(x, y, 128, gs, pcs);
            x += 1;
        }
        y += 1;
    }
    charpic(4, 14, player, east, 2, gs, pcs);
    charpic(19, 15, shot, east, 1, gs, pcs);
    charpic(17, 14, shot, east, 0, gs, pcs);
    charpic(15, 15, shot, east, 1, gs, pcs);
    charpic(8, 14, bigshot, east, 0, gs, pcs);
    if !wantmore(gs, pcs, pas, sdl) {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    print_str("Button 2 / alt key:\n", gs, pcs);
    print_str("Allows you to move  \n", gs, pcs);
    print_str("without changing the\n", gs, pcs);
    print_str("direction you are   \n", gs, pcs);
    print_str("facing.  Good for   \n", gs, pcs);
    print_str("searching walls and \n", gs, pcs);
    print_str("fighting retreats.  \n", gs, pcs);
    y = 11;
    while y <= 18 {
        x = 3;
        while x <= 20 {
            if y == 15 {
                drawchartile(x, y, 129, gs, pcs);
            } else if y == 16 {
                drawchartile(x, y, 131, gs, pcs);
            } else {
                drawchartile(x, y, 128, gs, pcs);
            }
            x += 1;
        }
        y += 1;
    }
    charpic(6, 13, player, south, 2, gs, pcs);
    pcs.sx = 6;
    pcs.sy = 15;
    print_str("\x1D\x1D\x1E\x1E\x1F\x1F", gs, pcs);
    if !wantmore(gs, pcs, pas, sdl) {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    print_str("\"P\" or \"space\" will \n", gs, pcs);
    print_str("take a healing      \n", gs, pcs);
    print_str("potion if you have  \n", gs, pcs);
    print_str("one.  This restores \n", gs, pcs);
    print_str("the body meter to   \n", gs, pcs);
    print_str("full strength.  Keep\n", gs, pcs);
    print_str("a sharp eye on the  \n", gs, pcs);
    print_str("meter, because when \n", gs, pcs);
    print_str("it runs out, you are\n", gs, pcs);
    print_str("dead!               \n\n", gs, pcs);
    print_str("\"B\" will cast a bolt\n", gs, pcs);
    print_str("spell if you have   \n", gs, pcs);
    print_str("any.  You can mow   \n", gs, pcs);
    print_str("down a lot of       \n", gs, pcs);
    print_str("monsters with a bit \n", gs, pcs);
    print_str("of skill.           \n", gs, pcs);
    if !wantmore(gs, pcs, pas, sdl) {
        return;
    }
    centerwindow(20, 20, gs, pcs);
    print_str("\"N\" or \"enter\" will \n", gs, pcs);
    print_str("cast a nuke spell.  \n", gs, pcs);
    print_str("This usually wipes  \n", gs, pcs);
    print_str("out all the monsters\n", gs, pcs);
    print_str("near you.  Consider \n", gs, pcs);
    print_str("it a panic button   \n", gs, pcs);
    print_str("when you are being  \n", gs, pcs);
    print_str("mobbed by monsters! \n\n", gs, pcs);
    printchartile(b"               \x80\x80\x80\n\0", gs, pcs);
    printchartile(b"POTIONS:       \x80\xA2\x80\n\0", gs, pcs);
    printchartile(b"               \x80\x80\x80\n\0", gs, pcs);
    printchartile(b"SCROLLS:       \x80\xA3\x80\n\0", gs, pcs);
    printchartile(b" (BOLTS/NUKES) \x80\x80\x80\n\0", gs, pcs);
    printchartile(b"TREASURE:      \x80\xA7\x80\n\0", gs, pcs);
    printchartile(b" (POINTS)      \x80\x80\x80\n\0", gs, pcs);
    printchartile(b"               \x80\x80\x80\n\0", gs, pcs);
    wantmore(gs, pcs, pas, sdl);
}

/*       */
/* reset */
/*       */
#[allow(dead_code)]
fn reset(
    gs: &mut GlobalState,
    pcs: &mut PcrlibCState,
    pas: &mut PcrlibAState,
    sdl: &mut SdlManager,
) {
    centerwindow(18, 1, gs, pcs);
    print_str("reset game (y/n)?", gs, pcs);
    let ch = get(gs, pcs, pas, sdl) as i8;
    if ch == 'y' as i8 {
        gs.gamexit = killed;
        gs.playdone = true;
    }
}

pub fn loadlevel(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    let tokens: [classtype; 26] = [
        player, teleporter, goblin, skeleton, ogre, gargoyle, dragon, turbogre, guns, gune,
        secretgate, nothing, nothing, nothing, nothing, nothing, nothing, nothing, nothing,
        nothing, nothing, nothing, nothing, nothing, nothing, nothing,
    ];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut xx: i32 = 0;
    let mut yy: i32 = 0;
    let mut btile: u8 = 0;
    let mut sm = vec![];
    let mut rle = [0; 4096];
    let filename = format!("LEVEL{}.CA2", pcs.level);
    let filesize = loadFile(&filename, &mut rle);
    RLEExpand(&rle[4..], filesize, &mut sm);
    gs.numobj = 0;
    gs.o[0].x = 13;
    gs.o[0].y = 13;
    gs.o[0].stage = 0;
    gs.o[0].delay = 0;
    gs.o[0].dir = east as i32 as u16;
    gs.o[0].oldx = 0;
    gs.o[0].oldy = 0;
    gs.o[0].oldtile = -1_i16;
    yy = 0;
    while yy < 64 {
        xx = 0;
        while xx < 64 {
            btile = sm[(yy * 64 + xx) as usize] as u8;
            if (btile as i32) < 230 {
                gs.background[(yy + 11) as usize][(xx + 11) as usize] = btile as i32;
            } else {
                gs.background[(yy + 11) as usize][(xx + 11) as usize] = 128;
                if tokens[(btile as i32 - 230) as usize] as u32 == player as i32 as u32 {
                    gs.o[0].x = (xx + 11) as u8;
                    gs.o[0].y = (yy + 11) as u8;
                } else {
                    gs.numobj += 1;
                    gs.o[gs.numobj as usize].active = false;
                    gs.o[gs.numobj as usize].class = tokens[(btile as i32 - 230) as usize];
                    gs.o[gs.numobj as usize].x = (xx + 11) as u8;
                    gs.o[gs.numobj as usize].y = (yy + 11) as u8;
                    gs.o[gs.numobj as usize].stage = 0;
                    gs.o[gs.numobj as usize].delay = 0;
                    // Defensive typecast.
                    gs.o[gs.numobj as usize].dir = Into::<dirtype>::into(rndt(pas) / 64) as u16;
                    gs.o[gs.numobj as usize].hp =
                        gs.objdef[gs.o[gs.numobj as usize].class as usize].hitpoints as i8;
                    gs.o[gs.numobj as usize].oldx = gs.o[gs.numobj as usize].x;
                    gs.o[gs.numobj as usize].oldy = gs.o[gs.numobj as usize].y;
                    gs.o[gs.numobj as usize].oldtile = -1_i16;
                }
            }
            xx += 1;
        }
        yy += 1;
    }
    gs.origin.x = gs.o[0].x as i32 - 11;
    gs.origin.y = gs.o[0].y as i32 - 11;
    gs.shotpower = 0;
    y = 11 - 1;
    while y < 65 + 11 {
        x = 11 - 1;
        while x < 64 + 11 {
            gs.view[y as usize][x as usize] = gs.background[y as usize][x as usize];
            x += 1;
        }
        y += 1;
    }
    pcs.sx = 33;
    pcs.sy = 1;
    print_str(&pcs.level.to_string(), gs, pcs);
    print_str(" ", gs, pcs);
    restore(gs, pcs);
    i = 0;
    while i < 6 {
        gs.saveitems[i as usize] = gs.items[i as usize];
        i += 1;
    }
    gs.savescore = pcs.score;
    gs.saveo[0] = gs.o[0];
}

fn drawside(gs: &mut GlobalState, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    pcs.sx = 0;
    while pcs.sx < 40 {
        drawchar(pcs.sx, 24, 0, gs, pcs);
        pcs.sx += 1;
    }
    pcs.sy = 0;
    while pcs.sy < 24 {
        drawchar(39, pcs.sy, 0, gs, pcs);
        pcs.sy += 1;
    }
    drawwindow(24, 0, 38, 23, gs, pcs);
    print_str("  level\n\nscore:\n\ntop  :\n\nk:\np:\nb:\nn:\n\n", gs, pcs);
    print_str(" shot power\n\n\n    body\n\n\n", gs, pcs);
    printhighscore(gs, pcs);
    printbody(gs, pcs);
    printshotpower(gs, pcs);
    printscore(gs, pcs);
    pcs.sx = 33;
    pcs.sy = 1;
    print_str(&pcs.level.to_string(), gs, pcs);
    drawpic(25 * 8, 17 * 8, 13, gs, cps, pcs);
    i = 1;
    while i <= gs.items[1] as i32 && i < 11 {
        drawchar(26 + i, 7, 31, gs, pcs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[2] as i32 && i < 11 {
        drawchar(26 + i, 8, 29, gs, pcs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[3] as i32 && i < 11 {
        drawchar(26 + i, 9, 30, gs, pcs);
        i += 1;
    }
    i = 1;
    while i <= gs.items[5] as i32 && i < 11 {
        drawchar(26 + i, 10, 30, gs, pcs);
        i += 1;
    }
}

fn playsetup(gs: &mut GlobalState, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    let mut i: i32 = 0;
    gs.shotpower = 0;
    bar(0, 0, 23, 23, 0, gs, pcs);
    if pcs.level as i32 == 0 {
        i = 1;
        while i < 6 {
            gs.items[i as usize] = 0;
            i += 1;
        }
        pcs.score = 0;
        pcs.level = 1;
        gs.o[0].active = true;
        gs.o[0].class = player;
        gs.o[0].hp = 13;
        gs.o[0].dir = west as i32 as u16;
        gs.o[0].stage = 0;
        gs.o[0].delay = 0;
        drawside(gs, cps, pcs);
        givenuke(gs, pcs);
        givenuke(gs, pcs);
        givebolt(gs, pcs);
        givebolt(gs, pcs);
        givebolt(gs, pcs);
        givepotion(gs, pcs);
        givepotion(gs, pcs);
        givepotion(gs, pcs);
    } else {
        drawside(gs, cps, pcs);
    };
}

pub fn repaintscreen(gs: &mut GlobalState, cps: &mut CpanelState, pcs: &mut PcrlibCState) {
    match gs.gamestate {
        statetype::intitle => {
            drawpic(0, 0, 14, gs, cps, pcs);
        }
        statetype::ingame => {
            restore(gs, pcs);
            drawside(gs, cps, pcs);
            printscore(gs, pcs);
            pcs.sx = 33;
            pcs.sy = 1;
            print_str(&pcs.level.to_string(), gs, pcs);
        }
        statetype::inscores => {
            restore(gs, pcs);
            drawside(gs, cps, pcs);
            printscore(gs, pcs);
            pcs.sx = 33;
            pcs.sy = 1;
            print_str(&pcs.level.to_string(), gs, pcs);
            gs.indemo = demoplay;
        }
    };
}

/*
=============
=
= dofkeys
=
= Checks to see if an F-key is being pressed and handles it
=
=============
*/

pub fn dofkeys(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    let mut key = bioskey(1, pcs, pas, sdl);
    // make ESC into F10
    if key == SDL_SCANCODE_ESCAPE {
        key = SDL_SCANCODE_F10;
    }
    if key < SDL_SCANCODE_F1 || key > SDL_SCANCODE_F10 {
        return;
    }
    match key {
        // F1
        SDL_SCANCODE_F1 => {
            clearkeys(pcs, pas, sdl);
            help(gs, pcs, pas, sdl);
        }
        // F2
        SDL_SCANCODE_F2 => {
            clearkeys(pcs, pas, sdl);
            controlpanel(gs, cps, pas, pcs, sdl);
        }
        // F3
        SDL_SCANCODE_F3 => {
            clearkeys(pcs, pas, sdl);
            expwin(18, 1, gs, pas, pcs);
            print_str("RESET GAME (Y/N)?", gs, pcs);
            let ch = (get(gs, pcs, pas, sdl) as u8).to_ascii_uppercase() as i8;
            if ch as i32 == 'Y' as i32 {
                gs.resetgame = true;
            }
        }
        // F4
        SDL_SCANCODE_F4 => {
            clearkeys(pcs, pas, sdl);
            expwin(22, 4, gs, pas, pcs);
            if gs.indemo != notdemo {
                print_str("Can't save game here!", gs, pcs);
                get(gs, pcs, pas, sdl);
            } else {
                print_str("Save as game #(1-9):", gs, pcs);
                // Rust port: The upper casing is in the original, and it's redundant.
                let ch1 = (get(gs, pcs, pas, sdl) as u8).to_ascii_uppercase();
                drawchar(pcs.sx, pcs.sy, ch1 as i32, gs, pcs);
                if ch1 >= b'1' && ch1 <= b'9' {
                    let mut save_game = true;
                    //
                    // save game
                    //
                    // Rust port: Very easy to miss the subtraction!
                    let str = format!("GAME{}.CA2", ch1 - b'0');
                    if _Verify(&str) != 0 {
                        print_str("\nGame exists,\noverwrite (Y/N)?", gs, pcs);
                        let ch2 = get(gs, pcs, pas, sdl) as u8;
                        if ch2 != b'Y' && ch2 != b'y' {
                            save_game = false;
                        } else {
                            pcs.sx = pcs.leftedge;
                            print_str("                    ", gs, pcs);
                            pcs.sy -= 1;
                            pcs.sx = pcs.leftedge;
                            print_str("                    ", gs, pcs);
                            pcs.sx = pcs.leftedge;
                            pcs.sy -= 1;
                        }
                    }
                    if save_game {
                        // Rust port: Former flags: (O_WRONLY | O_BINARY | O_CREAT | O_TRUNC,
                        // S_IREAD | S_IWRITE).
                        if let Ok(mut file) = File::create(str) {
                            gs.saveitems.serialize(&mut file).unwrap();
                            gs.savescore.serialize(&mut file).unwrap();
                            pcs.level.serialize(&mut file).unwrap();
                            gs.saveo.serialize(&mut file).unwrap();

                            print_str("\nGame saved.  Hit F5\n", gs, pcs);
                            print_str("when you wish to\n", gs, pcs);
                            print_str("restart the game.", gs, pcs);
                            get(gs, pcs, pas, sdl);
                        } else {
                            return;
                        }
                    }
                }
            }
        }
        // F5
        SDL_SCANCODE_F5 => {
            clearkeys(pcs, pas, sdl);
            expwin(22, 4, gs, pas, pcs);
            print_str("Load game #(1-9):", gs, pcs);
            // Rust port: The upper casing is in the original, and it's redundant.
            let ch = (get(gs, pcs, pas, sdl) as u8).to_ascii_uppercase();
            drawchar(pcs.sx, pcs.sy, ch as i32, gs, pcs);
            if ch >= b'1' && ch <= b'9' {
                //
                // load game
                //
                // Rust port: Very easy to miss the subtraction!
                let str = format!("GAME{}.CA2", ch - b'0');
                // Rust port: Flags in the original port = (O_RDONLY | O_BINARY, S_IWRITE | S_IREAD);
                // oddly, O_RDONLY == O_BINARY == 0.
                if let Ok(mut file) = File::open(str) {
                    gs.items = Deserialize::deserialize(&mut file).unwrap();
                    pcs.score = Deserialize::deserialize(&mut file).unwrap();
                    pcs.level = Deserialize::deserialize(&mut file).unwrap();
                    gs.o[0] = Deserialize::deserialize(&mut file).unwrap();
                    gs.exitdemo = true;
                    if gs.indemo != notdemo {
                        gs.playdone = true;
                    }
                    drawside(gs, cps, pcs); // draw score, icons, etc
                    gs.leveldone = true;
                } else {
                    print_str("\nGame not found.", gs, pcs);
                    get(gs, pcs, pas, sdl);
                }
            }
        }
        // F9
        SDL_SCANCODE_F9 => {
            clearkeys(pcs, pas, sdl);
            expwin(7, 1, gs, pas, pcs);
            print_str("PAUSED", gs, pcs);
            get(gs, pcs, pas, sdl);
        }
        // F10
        SDL_SCANCODE_F10 => {
            clearkeys(pcs, pas, sdl);
            expwin(12, 1, gs, pas, pcs);
            print_str("QUIT (Y/N)?", gs, pcs);
            let ch = (get(gs, pcs, pas, sdl) as u8).to_ascii_uppercase() as i8;
            if ch == 'Y' as i8 {
                _quit(None, pas, pcs, sdl);
            }
        }
        _ => return,
    }

    clearold(&mut gs.oldtiles);
    clearkeys(pcs, pas, sdl);
    repaintscreen(gs, cps, pcs);
}

fn dotitlepage(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    let mut i: i32 = 0;
    drawpic(0, 0, 14, gs, cps, pcs);
    UpdateScreen(gs, pcs);
    gs.gamestate = statetype::intitle;
    i = 0;
    while i < 300 {
        WaitVBL();
        gs.indemo = notdemo;
        gs.ctrl = ControlPlayer(1, gs, pcs, pas, sdl);
        if gs.ctrl.button1 as i32 != 0
            || gs.ctrl.button2 as i32 != 0
            || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            pcs.level = 0;
            gs.exitdemo = true;
            break;
        } else {
            gs.indemo = demoplay;
            if bioskey(1, pcs, pas, sdl) != 0 {
                dofkeys(gs, cps, pas, pcs, sdl);
                UpdateScreen(gs, pcs);
            }
            if gs.exitdemo {
                break;
            }
            i += 1;
        }
    }
    gs.gamestate = statetype::ingame;
}

fn doendpage(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    WaitEndSound(gs, pas, pcs);
    drawpic(0, 0, 15, gs, cps, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    PlaySound(3, pas);
    WaitEndSound(gs, pas, pcs);
    drawwindow(0, 0, 17, 9, gs, pcs);
    print_str("Congratulation! \n", gs, pcs);
    print_str("One as skilled  \n", gs, pcs);
    print_str("as yourself     \n", gs, pcs);
    print_str("deserves the    \n", gs, pcs);
    print_str("10,000,000 gold \n", gs, pcs);
    print_str("you pulled out  \n", gs, pcs);
    print_str("of the palace! ", gs, pcs);
    clearkeys(pcs, pas, sdl);
    get(gs, pcs, pas, sdl);
    drawwindow(0, 0, 17, 9, gs, pcs);
    print_str("Let us know what\n", gs, pcs);
    print_str("you enjoyed     \n", gs, pcs);
    print_str("about this game,\n", gs, pcs);
    print_str("so we can give  \n", gs, pcs);
    print_str("you more of it. \n", gs, pcs);
    print_str("Thank you for   \n", gs, pcs);
    print_str("playing!", gs, pcs);
    get(gs, pcs, pas, sdl);
}

fn dodemo(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    let mut i: i32 = 0;
    while !gs.exitdemo {
        dotitlepage(gs, cps, pas, pcs, sdl);
        if gs.exitdemo {
            break;
        }
        i = rnd(NUM_DEMOS - 1, pas) + 1;
        LoadDemo(i, gs, pcs);
        pcs.level = 0;
        playsetup(gs, cps, pcs);
        playloop(gs, cps, pas, pcs, sdl);
        if gs.exitdemo {
            break;
        }
        pcs.level = 0;
        gs.gamestate = statetype::inscores;
        gs.indemo = demoplay;
        _showhighscores(gs, pcs);
        UpdateScreen(gs, pcs);
        i = 0;
        while i < 500 {
            WaitVBL();
            gs.indemo = notdemo;
            gs.ctrl = ControlPlayer(1, gs, pcs, pas, sdl);
            if gs.ctrl.button1 as i32 != 0
                || gs.ctrl.button2 as i32 != 0
                || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
            {
                gs.exitdemo = true;
                break;
            } else {
                if bioskey(1, pcs, pas, sdl) != 0 {
                    dofkeys(gs, cps, pas, pcs, sdl);
                }
                if gs.exitdemo {
                    break;
                }
                i += 1;
            }
        }
    }
}

fn gameover(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    let mut i: i32 = 0;
    expwin(11, 4, gs, pas, pcs);
    print_str("\n GAME OVER\n     ", gs, pcs);
    UpdateScreen(gs, pcs);
    WaitEndSound(gs, pas, pcs);
    i = 0;
    while i < 120 {
        WaitVBL();
        i += 1;
    }
    gs.gamestate = statetype::inscores;
    _checkhighscore(gs, pas, pcs, sdl);
    pcs.level = 0;
    i = 0;
    while i < 500 {
        WaitVBL();
        gs.ctrl = ControlPlayer(1, gs, pcs, pas, sdl);
        if gs.ctrl.button1 as i32 != 0
            || gs.ctrl.button2 as i32 != 0
            || pcs.keydown[SDL_SCANCODE_SPACE as usize] as i32 != 0
        {
            break;
        }
        if bioskey(1, pcs, pas, sdl) != 0 {
            dofkeys(gs, cps, pas, pcs, sdl);
        }
        if gs.exitdemo as i32 != 0 || gs.indemo == demoplay {
            break;
        }
        i += 1;
    }
}

/***************************************************************************/
/***************************************************************************/

/*=========================*/
/*			   */
/* m a i n   p r o g r a m */
/*			   */
/*=========================*/

pub fn original_main() {
    // Rust port: The SDL/Event watch initializations have been moved here, since they must stay in
    // the global scope.
    let mut sdl = SdlManager::init_sdl();

    // Rust port: This needs to be on the global scope, because `Timer` lifetime(s) are bound to it;
    // if it's placed inside the SdlManager, the lifetime(s) will be bound to the manager.
    let timer_sys = sdl.timer();

    // Rust port: Option<TextureCreator<_>> is a workaround necessary to allow Texture live within
    // PcrlibCState, as a texture's lifetime is bound to its texture creator, which therefore needs
    // to be in a higher scope; this is a problem because both the variables TextureCreator depends
    // on, and Texture, are inside PcrlibCState. The clean alternative is to move the texture out of
    // PcrlibCState and pass it around, which is not great, considering the amount of state
    // variables already passed around.
    let mut texture_creator = None;

    // Rust port: Globals

    let mut gs = GlobalState::default();
    let mut cps = CpanelState::default();
    let mut pas = PcrlibAState::new();

    /***************************************************************************/

    let ver_arg_position = std::env::args().position(|arg| arg == "/VER");

    if let Some(1) = ver_arg_position {
        print!(
            "\
                    CatacombSDL\n\
                    Version 1.03\n
\
                    Usage: catacomb [windowed <width> <height>] [screen <num>]\n
\
                    Ported by Braden \"Blzut3\" Obrzut and Rene \"Havoc\" Nicolaus\n\
                    Includes PC Speaker emulator by K1n9_Duk3\n\
                    Based on The Catacomb source code:\n\
                    Copyright 1990-1993 Softdisk Publishing\n\
                    Copyright 1993-2014 Flat Rock Software\n\
                "
        );
        std::process::exit(0);
    }

    initobjects(&mut gs.objdef);

    gs.priority.fill(99);

    gs.priority[blankfloor] = 0;
    for i in
        gs.objdef[teleporter as usize].firstchar..=gs.objdef[teleporter as usize].firstchar + 20
    {
        gs.priority[i as usize] = 0;
    }
    for clvar in (dead2 as usize)..=(dead5 as usize) {
        for i in gs.objdef[clvar].firstchar
            ..=(gs.objdef[clvar].firstchar
                + gs.objdef[clvar].size as u16 * gs.objdef[clvar].size as u16)
        {
            gs.priority[i as usize] = 0; /*deadthing*/
        }
    }
    for i in 152..=161 {
        gs.priority[i] = 2; /*shots*/
    }
    for i in gs.objdef[bigshot as usize].firstchar..=(gs.objdef[bigshot as usize].firstchar + 31) {
        gs.priority[i as usize] = 2; /*bigshot*/
    }
    for i in 0..=(tile2s - 1) {
        if gs.priority[i] == 99 {
            gs.priority[i] = 3; /*most 1*1 tiles are walls, etc*/
        }
    }
    gs.priority[167] = 1; // chest
    for i in tile2s..=maxpics {
        if gs.priority[i] as i32 == 99 {
            gs.priority[i] = 4; /*most bigger tiles are monsters*/
        }
    }
    for i in gs.objdef[player as usize].firstchar..=(gs.objdef[player as usize].firstchar + 63) {
        gs.priority[i as usize] = 5; /*player*/
    }

    gs.side = 0;

    for x in 0..=85 {
        for y in 0..=(topoff - 1) {
            gs.view[x][y] = solidwall;
            gs.view[x][(85 - y)] = solidwall;
            gs.background[x][y] = solidwall;
            gs.background[x][(85 - y)] = solidwall;
        }
        gs.view[86][x] = solidwall;
    }
    for y in 11..=74 {
        for x in 0..=(leftoff - 1) {
            gs.view[x][y] = solidwall;
            gs.view[(85 - x)][y] = solidwall;
            gs.background[x][y] = solidwall;
            gs.background[(85 - x)][y] = solidwall;
        }
    }

    //   puts ("CATACOMB II is executing");

    //  _dontplay = 1;	// no sounds for debugging and profiling

    let (mut pcs, _vbl_timer, _audio_dev) = _setupgame(
        &mut gs,
        &mut cps,
        &mut pas,
        &sdl,
        &mut texture_creator,
        &timer_sys,
    );

    expwin(33, 13, &mut gs, &mut pas, &mut pcs);
    print_str("  Softdisk Publishing presents\n\n", &mut gs, &mut pcs);
    print_str("          The Catacomb\n\n", &mut gs, &mut pcs);
    print_str("        By John Carmack\n\n", &mut gs, &mut pcs);
    print_str("       Copyright 1990-93\n", &mut gs, &mut pcs);
    print_str("      Softdisk Publishing", &mut gs, &mut pcs);
    print_str("\n\n", &mut gs, &mut pcs);
    print_str("\n\n", &mut gs, &mut pcs);
    print_str("         Press a key:", &mut gs, &mut pcs);
    get(&mut gs, &mut pcs, &mut pas, &mut sdl);

    clearkeys(&mut pcs, &mut pas, &mut sdl);

    gs.screencenter.x = 11;
    gs.screencenter.y = 11;

    gs.exitdemo = false;
    pcs.level = 0;

    // go until quit () is called
    loop {
        dodemo(&mut gs, &mut cps, &mut pas, &mut pcs, &mut sdl);
        playsetup(&mut gs, &mut cps, &mut pcs);
        gs.indemo = notdemo;
        gs.gamestate = statetype::ingame;
        playloop(&mut gs, &mut cps, &mut pas, &mut pcs, &mut sdl);
        if gs.indemo == notdemo {
            gs.exitdemo = false;
            if pcs.level > numlevels {
                doendpage(&mut gs, &mut cps, &mut pas, &mut pcs, &mut sdl); // finished all levels
            }
            gameover(&mut gs, &mut cps, &mut pas, &mut pcs, &mut sdl);
        }
    }
}
