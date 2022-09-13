use num::Integer;

use crate::{
    cat_play::{doactive, doinactive},
    catacomb::refresh,
    class_type::classtype::*,
    cpanel_state::CpanelState,
    global_state::GlobalState,
    gr_type::grtype::*,
    pcrlib_a::{screenpitch, EGA},
    pcrlib_a_state::PcrlibAState,
    pcrlib_c::UpdateScreen,
    pcrlib_c_state::PcrlibCState,
    sdl_manager::SdlManager,
};

//========================================================================

//=========================================
//
// DRAWOBJ
// Draws the object to TILES in the proper
// direction and state.
//
//=========================================

const squares: [u8; 9] = [0, 1, 4, 9, 16, 25, 36, 49, 64];

const table86: [u16; 87] = [
    0, 86, 172, 258, 344, 430, 516, 602, 688, 774, 860, 946, 1032, 1118, 1204, 1290, 1376, 1462,
    1548, 1634, 1720, 1806, 1892, 1978, 2064, 2150, 2236, 2322, 2408, 2494, 2580, 2666, 2752, 2838,
    2924, 3010, 3096, 3182, 3268, 3354, 3440, 3526, 3612, 3698, 3784, 3870, 3956, 4042, 4128, 4214,
    4300, 4386, 4472, 4558, 4644, 4730, 4816, 4902, 4988, 5074, 5160, 5246, 5332, 5418, 5504, 5590,
    5676, 5762, 5848, 5934, 6020, 6106, 6192, 6278, 6364, 6450, 6536, 6622, 6708, 6794, 6880, 6966,
    7052, 7138, 7224, 7310, 7396,
];

pub fn drawobj(gs: &mut GlobalState) {
    let mut tilenum = gs.obj.firstchar as i32
        + squares[gs.obj.size as usize] as i32
            * ((gs.obj.dir as i32 & gs.obj.dirmask as i32) * gs.obj.stages as i32
                + gs.obj.stage as i32);
    gs.obj.oldtile = tilenum as i16;
    gs.obj.oldy = gs.obj.y;
    gs.obj.oldx = gs.obj.x;

    let objpri = gs.priority[tilenum as usize]; // entire object has same priority
    let mut ofs = (table86[gs.obj.oldy as usize] as i32 + gs.obj.oldx as i32) as usize; // View is 86*86

    for _y in 0..gs.obj.size {
        for _x in 0..gs.obj.size {
            let (ofs_row, ofs_col) = ofs.div_mod_floor(&86);
            let view_obj = &mut gs.view[ofs_row][ofs_col];
            // check tiles priority level
            // don't draw if lower than what's there
            if gs.priority[*view_obj as usize] <= objpri {
                *view_obj = tilenum;
            }
            tilenum += 1;
            ofs += 1;
        }
        // position destination at start of next line
        ofs += 86 - gs.obj.size as usize;
    }
}

//=======================================================================

//=======================================
//
// ERASEOBJ
// Erases the current object by copying
// the background onto the view where the
// object is standing
//
//=======================================

pub fn eraseobj(gs: &mut GlobalState) {
    // only erase chars that match what was drawn by the last drawobj
    let mut tilenum = gs.obj.oldtile;

    let mut ofs = (table86[gs.obj.oldy as usize] as i32 + gs.obj.oldx as i32) as usize; // View is 86*86

    for _y in 0..gs.obj.size {
        for _x in 0..gs.obj.size {
            let (ofs_row, ofs_col) = ofs.div_mod_floor(&86);
            let view_obj = &mut gs.view[ofs_row][ofs_col];
            // don't erase if its not part of the shape
            if *view_obj == tilenum as i32 {
                *view_obj = gs.background[ofs_row][ofs_col]; // erase it
            }
            tilenum += 1;
            ofs += 1;
        }
        // position destination at start of next line
        ofs += 86 - gs.obj.size as usize;
    }
}

pub fn doall(
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pas: &mut PcrlibAState,
    pcs: &mut PcrlibCState,
    sdl: &mut SdlManager,
) {
    assert!(gs.numobj > 0);

    loop {
        gs.objecton = gs.numobj;
        loop {
            gs.obj.update_from_activeobj(&gs.o[gs.objecton as usize]);
            if gs.obj.class as i32 != nothing as i32 {
                gs.obj
                    .update_from_objdeftype(&gs.objdef[gs.obj.class as usize]);
                if gs.obj.active {
                    doactive(gs, cps, pas, pcs, sdl);
                } else {
                    doinactive(gs);
                }
            }
            if gs.leveldone || gs.playdone {
                return;
            }
            gs.objecton -= 1;
            if !(gs.objecton >= 0) {
                break;
            }
        }
        refresh(gs, pcs);
        gs.frameon = gs.frameon.wrapping_add(1);
        if gs.leveldone {
            return;
        }
        if gs.playdone {
            break;
        }
    }
}

fn drawcgachartile(screenseg_ofs: usize, tile: i32, gs: &mut GlobalState) {
    let mut src = &gs.pics[(tile << 4) as usize..];
    let mut dest = &mut gs.screenseg[screenseg_ofs..];

    for _ in 0..8 {
        dest[0] = src[0] >> 6 & 3;
        dest = &mut dest[1..];
        dest[0] = src[0] >> 4 & 3;
        dest = &mut dest[1..];
        dest[0] = src[0] >> 2 & 3;
        dest = &mut dest[1..];
        dest[0] = src[0] >> 0 & 3;
        dest = &mut dest[1..];
        dest[0] = src[1] >> 6 & 3;
        dest = &mut dest[1..];
        dest[0] = src[1] >> 4 & 3;
        dest = &mut dest[1..];
        dest[0] = src[1] >> 2 & 3;
        dest = &mut dest[1..];
        dest[0] = src[1] >> 0 & 3;

        dest = &mut dest[(screenpitch - 7) as usize..];

        src = &src[2..];
    }
}

//=========
//
// CGAREFRESH redraws the tiles that have changed in the tiled screen area
//
//=========

pub fn cgarefresh(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut ofs = (gs.origin.y * 86 + gs.origin.x) as usize;

    let mut i = 0;
    let mut endofrow = ofs + 24;
    let mut screenseg_ofs = 0;
    loop {
        let (ofs_row, ofs_col) = ofs.div_mod_floor(&86);
        let tile = gs.view[ofs_row][ofs_col];
        if tile != gs.oldtiles[i] {
            gs.oldtiles[i] = tile;
            drawcgachartile(screenseg_ofs, tile, gs);
        }
        i += 1;
        ofs += 1;
        screenseg_ofs += 8;

        if ofs == endofrow {
            if i == 24 * 24 {
                break;
            }
            ofs += 86 - 24;
            endofrow += 86;
            screenseg_ofs += screenpitch * 8 - 24 * 8;
        }
    }

    UpdateScreen(gs, pcs);
}

fn drawegachartile(screenseg_ofs: usize, tile: i32, gs: &mut GlobalState) {
    let src = &gs.pics;
    let dest = &mut gs.screenseg;

    let mut src_i = (tile << 5) as usize;
    let mut dest_i = screenseg_ofs;

    for _ in 0..8 {
        let chan: [u8; 4] = [
            src[src_i + 0],
            src[src_i + 8],
            src[src_i + 16],
            src[src_i + 24],
        ];

        dest[dest_i] = EGA(&chan, 7);
        dest_i += 1;
        dest[dest_i] = EGA(&chan, 6);
        dest_i += 1;
        dest[dest_i] = EGA(&chan, 5);
        dest_i += 1;
        dest[dest_i] = EGA(&chan, 4);
        dest_i += 1;
        dest[dest_i] = EGA(&chan, 3);
        dest_i += 1;
        dest[dest_i] = EGA(&chan, 2);
        dest_i += 1;
        dest[dest_i] = EGA(&chan, 1);
        dest_i += 1;
        dest[dest_i] = EGA(&chan, 0);

        src_i += 1;
        dest_i += screenpitch - 7;
    }
}

//=========
//
// EGAREFRESH redraws the tiles that have changed in the tiled screen area
//
//=========

// Rust port: Identical to cgarefresh, with the exception that it calls drawegachartile().
pub fn egarefresh(gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let mut ofs = (gs.origin.y * 86 + gs.origin.x) as usize;

    let mut i = 0;
    let mut endofrow = ofs + 24;
    let mut screenseg_ofs = 0;
    loop {
        let (ofs_row, ofs_col) = ofs.div_mod_floor(&86);
        let tile = gs.view[ofs_row][ofs_col];
        if tile != gs.oldtiles[i] {
            gs.oldtiles[i] = tile;
            drawegachartile(screenseg_ofs, tile, gs);
        }
        i += 1;
        ofs += 1;
        screenseg_ofs += 8;

        if ofs == endofrow {
            if i == 24 * 24 {
                break;
            }
            ofs += 86 - 24;
            endofrow += 86;
            screenseg_ofs += screenpitch * 8 - 24 * 8;
        }
    }

    UpdateScreen(gs, pcs);
}

pub fn drawchartile(x: i32, y: i32, tile: i32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    match pcs.grmode {
        CGAgr => {
            drawcgachartile(
                ((y << 3) * screenpitch as i32 + (x << 3)) as usize,
                tile,
                gs,
            );
        }
        EGAgr | _ => {
            drawegachartile(
                ((y << 3) * screenpitch as i32 + (x << 3)) as usize,
                tile,
                gs,
            );
        }
    };
}
