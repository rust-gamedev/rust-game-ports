use std::{
    sync::{Condvar, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    timer::Timer,
    TimerSubsystem,
};

use crate::{
    cpanel_state::CpanelState,
    global_state::GlobalState,
    gr_type::grtype::*,
    pcrlib_a_state::{PcrlibAState, PcrlibAStateExclusive},
    pcrlib_c::UpdateScreen,
    pcrlib_c_state::PcrlibCState,
    sdl_manager::SdlManager,
    sound_type::soundtype::*,
    spkr_table::SPKRtable,
};

const PC_BASE_TIMER: u32 = 1193181;

// Rust port: Simulation of the SDL Semaphore
static vblSemMutex: Mutex<u32> = Mutex::new(0);
static vblSemCondvar: Condvar = Condvar::new();

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SavedSoundStruct {
    pub SndPriority: u8,
    pub pcSamplesPerTick: u32,
    pub pcLengthLeft: u32,
    // Rust port: Pointer to SoundData.freqdata
    pub pcSound: Option<usize>,
}

pub const screenpitch: usize = 320;
const VBL_TIME: u32 = 14;

#[inline]
pub fn EGA(chan: &[u8], ofs: u8) -> u8 {
    (chan[3] >> ofs & 1) << 3
        | (chan[2] >> ofs & 1) << 2
        | (chan[1] >> ofs & 1) << 1
        | (chan[0] >> ofs & 1) << 0
}

#[inline]
fn _SDL_turnOnPCSpeaker(pcSample: u16, pas: &mut PcrlibAStateExclusive) {
    // There is a bug in the SDL port; the data types used don't cover the range of values.
    // See [here](https://github.com/Blzut3/CatacombSDL/issues/4).
    //
    pas.pcPhaseLength = pcSample as u32 * pas.AudioSpecFreq as u32 / (2 * PC_BASE_TIMER);
    pas.pcActive = true;
}

#[inline]
fn _SDL_turnOffPCSpeaker(pas: &mut PcrlibAStateExclusive) {
    pas.pcActive = false;
    pas.pcPhaseTick = 0;
}

#[inline]
fn _SDL_PCService(pas: &mut PcrlibAStateExclusive) {
    if let Some(pcSound) = pas.pcSound {
        let pcCurrSample = pas.SoundData.freqdata[pcSound];

        if pcCurrSample != pas.pcLastSample {
            pas.pcLastSample = pcCurrSample;
            if pas.pcLastSample != 0 {
                _SDL_turnOnPCSpeaker(pas.pcLastSample, pas);
            } else {
                _SDL_turnOffPCSpeaker(pas);
            }
        }
        pas.pcSound = Some(pcSound + 1);
        pas.pcLengthLeft -= 1;
        if pas.pcLengthLeft == 0 {
            pas.pcSound = None;
            pas.SndPriority = 0;
            _SDL_turnOffPCSpeaker(pas);
        }
    }
}

fn _SDL_PCPlaySound(sound_i: i32, pas: &mut PcrlibAStateExclusive) {
    pas.pcPhaseTick = 0;
    pas.pcLastSample = 0;
    pas.pcLengthLeft = ((pas.SoundData.sounds[sound_i as usize].start as i32
        - pas.SoundData.sounds[(sound_i - 1) as usize].start as i32)
        >> 1) as u32;
    let sound_data_i = pas.SoundData.sounds[(sound_i - 1) as usize].start as usize
        - SPKRtable::start_of_freqdata();
    pas.pcSound = Some(sound_data_i / 2);
    pas.SndPriority = pas.SoundData.sounds[(sound_i - 1) as usize].priority;
    pas.pcSamplesPerTick = (pas.AudioSpecFreq
        / ((1193181 * pas.SoundData.sounds[(sound_i - 1) as usize].samplerate as i32) >> 16))
        as u32;
}

fn _SDL_PCStopSound(pasx: &mut PcrlibAState) {
    pasx.lock(|pas| {
        pas.pcSound = None;
    })
}

///////////////////////////////////////////////////////////////////////////
//
//      SDL_PCSpeakerEmulator() - Emulates the pc speaker
//      (replaces SDL_IMFMusicPlayer if no AdLib emulator is present)
//
///////////////////////////////////////////////////////////////////////////
pub struct Sound {
    pas: PcrlibAState,
}

impl AudioCallback for Sound {
    type Channel = i16;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.pas.lock(|pasx| {
            if pasx.soundmode != spkr {
                out.fill(0);
                return;
            }

            let mut pcNumReadySamples = pasx.pcSamplesPerTick;
            let out_len = out.len();

            for (i, w) in out.iter_mut().enumerate() {
                if pasx.pcActive {
                    *w = pasx.pcVolume;

                    if pasx.pcPhaseTick >= pasx.pcPhaseLength {
                        pasx.pcVolume = -pasx.pcVolume;
                        pasx.pcPhaseTick = 0;
                    } else {
                        pasx.pcPhaseTick += 1;
                    }
                } else {
                    *w = 0;
                }

                pcNumReadySamples -= 1;

                // Rust port: The conditionals below match the logic of the SDL port, where the inner block
                // is not executed when the buffer is emptied.
                // This condition `(i < ount.len() - 1)` should possibly (but not 100% sure) be removed, as
                // sound (sample) should still be progressed also on the last last buffer write.
                //
                if i < out_len - 1 {
                    if pcNumReadySamples == 0 {
                        _SDL_PCService(pasx);
                        pcNumReadySamples = pasx.pcSamplesPerTick;
                    }
                }
            }
        });
    }
}

/*
=============================================================================
======================== End of PC Speaker emulator ========================
=============================================================================
*/

//========
//
// StartupSound
//
//========

pub fn StartupSound(pas: &mut PcrlibAState, sdl: &SdlManager) -> Option<AudioDevice<Sound>> {
    let pas_clone = pas.clone();

    let audio_dev = pas.lock(|pasx| {
        let desired = AudioSpecDesired {
            freq: Some(48000),
            channels: Some(1),
            samples: Some(4096),
        };
        let pas_clone = pas_clone.clone();

        let audio_dev = sdl
            .audio()
            .open_playback(None, &desired, |_spec| Sound { pas: pas_clone });

        match audio_dev {
            Err(err) => {
                println!("Audio initialization failed: {}", err);
                pasx.soundmode = off;
                None
            }
            Ok(audio_dev) => {
                pasx.AudioSpecFreq = audio_dev.spec().freq;
                // Typical value for init since samplerate is usually 8
                pasx.pcSamplesPerTick = (pasx.AudioSpecFreq / 145) as u32;
                pasx.soundmode = spkr;
                audio_dev.resume();
                Some(audio_dev)
            }
        }
    });

    if audio_dev.is_none() {
        pas._dontplay = true;
    }

    audio_dev
}

pub fn PlaySound(sound: i32, pas: &mut PcrlibAState) {
    if pas._dontplay {
        return;
    }
    pas.lock(|pasx| {
        if pasx.SoundData.sounds[(sound - 1) as usize].priority as i32 >= pasx.SndPriority as i32 {
            _SDL_PCPlaySound(sound, pasx);
        }
    });
}

// Rust port: unused.
//
// fn StopSound(pas: &mut PcrlibAState) {
//     if pas._dontplay {
//         return;
//     }
//     _SDL_PCStopSound(pas);
// }

pub fn PauseSound(pas: &mut PcrlibAState) {
    if pas._dontplay {
        return;
    }
    pas.lock(|pasx| {
        pasx.SavedSound.SndPriority = pasx.SndPriority;
        pasx.SavedSound.pcSamplesPerTick = pasx.pcSamplesPerTick;
        pasx.SavedSound.pcLengthLeft = pasx.pcLengthLeft;
        pasx.SavedSound.pcSound = pasx.pcSound;
        pasx.SndPriority = 0;
        pasx.pcLengthLeft = 0;
        pasx.pcSound = None;
        _SDL_turnOffPCSpeaker(pasx);
    });
}

pub fn ContinueSound(pas: &mut PcrlibAState) {
    if pas._dontplay {
        return;
    }
    pas.lock(|pasx| {
        pasx.pcPhaseTick = 0;
        pasx.pcLastSample = 0;
        pasx.SndPriority = pasx.SavedSound.SndPriority;
        pasx.pcSamplesPerTick = pasx.SavedSound.pcSamplesPerTick;
        pasx.pcLengthLeft = pasx.SavedSound.pcLengthLeft;
        pasx.pcSound = pasx.SavedSound.pcSound;
    });
}

pub fn WaitEndSound(gs: &mut GlobalState, pas: &mut PcrlibAState, pcs: &mut PcrlibCState) {
    if pas._dontplay {
        return;
    }
    UpdateScreen(gs, pcs);
    while pas.lock(|pasx| pasx.pcSound.is_some()) {
        WaitVBL();
    }
}
const rndtable: [u8; 256] = [
    0, 8, 109, 220, 222, 241, 149, 107, 75, 248, 254, 140, 16, 66, 74, 21, 211, 47, 80, 242, 154,
    27, 205, 128, 161, 89, 77, 36, 95, 110, 85, 48, 212, 140, 211, 249, 22, 79, 200, 50, 28, 188,
    52, 140, 202, 120, 68, 145, 62, 70, 184, 190, 91, 197, 152, 224, 149, 104, 25, 178, 252, 182,
    202, 182, 141, 197, 4, 81, 181, 242, 145, 42, 39, 227, 156, 198, 225, 193, 219, 93, 122, 175,
    249, 0, 175, 143, 70, 239, 46, 246, 163, 53, 163, 109, 168, 135, 2, 235, 25, 92, 20, 145, 138,
    77, 69, 166, 78, 176, 173, 212, 166, 113, 94, 161, 41, 50, 239, 49, 111, 164, 70, 60, 2, 37,
    171, 75, 136, 156, 11, 56, 42, 146, 138, 229, 73, 146, 77, 61, 98, 196, 135, 106, 63, 197, 195,
    86, 96, 203, 113, 101, 170, 247, 181, 113, 80, 250, 108, 7, 255, 237, 129, 226, 79, 107, 112,
    166, 103, 241, 24, 223, 239, 120, 198, 58, 60, 82, 128, 3, 184, 66, 143, 224, 145, 224, 81,
    206, 163, 45, 63, 90, 168, 114, 59, 33, 159, 95, 28, 139, 123, 98, 125, 196, 15, 70, 194, 253,
    54, 14, 109, 226, 71, 17, 161, 93, 186, 87, 244, 138, 20, 52, 123, 251, 26, 36, 17, 46, 52,
    231, 232, 76, 31, 221, 84, 37, 216, 165, 212, 106, 197, 242, 98, 43, 39, 175, 254, 145, 190,
    84, 118, 222, 187, 136, 120, 163, 236, 249,
];

const baseRndArray: [u16; 17] = [
    1, 1, 2, 3, 5, 8, 13, 21, 54, 75, 129, 204, 323, 527, 850, 1377, 2227,
];

pub fn initrnd(randomize: bool, pas: &mut PcrlibAState) {
    pas.RndArray.copy_from_slice(&baseRndArray);
    pas.LastRnd = 0;
    pas.indexi = 17;
    pas.indexj = 5;
    if randomize {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        pas.RndArray[16] = (now & 0xffff) as u16;
        pas.RndArray[4] = (now & 0xffff ^ now >> 16 & 0xffff) as u16;
    }
    rnd(0xffff_i32 as u16, pas);
}

pub fn rnd(maxval: u16, pas: &mut PcrlibAState) -> i32 {
    let mut mask: u16 = 0;
    let mut shift: u16 = 0;
    let mut val: i32 = 0;
    if maxval as i32 == 0 {
        return 0;
    }
    mask = 0xffff_i32 as u16;
    shift = maxval;
    while shift as i32 & 0x8000_i32 == 0 {
        shift = ((shift as i32) << 1) as u16;
        mask = (mask as i32 >> 1) as u16;
    }
    val = pas.RndArray[(pas.indexi as i32 - 1) as usize] as i32
        + pas.RndArray[(pas.indexj as i32 - 1) as usize] as i32
        + 1;
    pas.RndArray[(pas.indexi as i32 - 1) as usize] = val as u16;
    val += pas.LastRnd as i32;
    pas.LastRnd = val as u16;
    pas.indexi = pas.indexi.wrapping_sub(1);
    if pas.indexi as i32 == 0 {
        pas.indexi = 17;
    }
    pas.indexj = pas.indexj.wrapping_sub(1);
    if pas.indexj as i32 == 0 {
        pas.indexj = 17;
    }
    val &= mask as i32;
    if val > maxval as i32 {
        val >>= 1;
    }
    val
}

pub fn initrndt(randomize: bool, pas: &mut PcrlibAState) {
    pas.rndindex = (if randomize as i32 != 0 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    } else {
        0
    }) as u16;
}

pub fn rndt(pas: &mut PcrlibAState) -> i32 {
    pas.rndindex = ((pas.rndindex as i32 + 1) & 0xff_i32) as u16;
    rndtable[pas.rndindex as usize] as i32
}

fn VBLCallback() -> u32 {
    let mut guard = vblSemMutex.lock().unwrap();

    *guard += 1;
    vblSemCondvar.notify_one();

    VBL_TIME
}

// In the SDL port, this was registered on atexit. Although it's tidy, it's not necessary, since (SQL)
// quit events (e.g. window closing) are trapped by SDL and handled by the WatchUIEvents.
// The only case where this can run is probably an unexpected termination. Since it's not strictly
// necessary anyway, and the cost is to require globals (atexit() doesn't support parameters), it
// can be safely removed.
//
// pub unsafe extern "C" fn ShutdownEmulatedVBL() {
//     safe_SDL_RemoveTimer(pas.vbltimer);
//     safe_SDL_DestroySemaphore(pas.vblsem);
// }

pub fn SetupEmulatedVBL(timer_sys: &TimerSubsystem) -> Timer<'_, '_> {
    // Rust port: No need to create the semaphore here

    timer_sys.add_timer(VBL_TIME, Box::new(VBLCallback))

    // Disabled; see comment on ShutdownEmulatedVBL().
    // safe_register_shutdown_vbl_on_exit();
}

pub fn WaitVBL() {
    let mut guard = vblSemMutex.lock().unwrap();

    loop {
        if *guard > 0 {
            *guard -= 1;
            break;
        } else {
            guard = vblSemCondvar.wait(guard).unwrap();
        }
    }
}

pub fn drawchar(x: i32, y: i32, charnum: i32, gs: &mut GlobalState, pcs: &mut PcrlibCState) {
    let src = &pcs.picfile_data;
    let mut src_i = pcs.charptr;

    let vbuf = &mut gs.screenseg;
    let mut vbuf_i = (((y as usize) << 3) * screenpitch) + ((x as usize) << 3);

    match pcs.grmode {
        CGAgr => {
            src_i += charnum as usize * 16;

            for _ in 0..8 {
                vbuf[vbuf_i] = src[src_i] >> 6 & 3;
                vbuf_i += 1;
                vbuf[vbuf_i] = src[src_i] >> 4 & 3;
                vbuf_i += 1;
                vbuf[vbuf_i] = src[src_i] >> 2 & 3;
                vbuf_i += 1;
                vbuf[vbuf_i] = src[src_i] >> 0 & 3;
                vbuf_i += 1;
                vbuf[vbuf_i] = src[src_i + 1] >> 6 & 3;
                vbuf_i += 1;
                vbuf[vbuf_i] = src[src_i + 1] >> 4 & 3;
                vbuf_i += 1;
                vbuf[vbuf_i] = src[src_i + 1] >> 2 & 3;
                vbuf_i += 1;
                vbuf[vbuf_i] = src[src_i + 1] >> 0 & 3;

                src_i += 2;
                vbuf_i += screenpitch - 7;
            }
        }
        VGAgr => {
            src_i += charnum as usize * 64;

            for _ in 0..8 {
                // [BL] More or less guessing here since we don't have VGA files to
                // test against.
                vbuf[vbuf_i..vbuf_i + 8].copy_from_slice(&src[src_i..src_i + 8]);

                src_i += 8;
                vbuf_i += screenpitch - 7;
            }
        }
        EGAgr | _ => {
            src_i += charnum as usize * 8;

            for _ in 0..8 {
                let chan: [u8; 4] = [
                    src[src_i + pcs.egaplaneofs[0] as usize],
                    src[src_i + pcs.egaplaneofs[1] as usize],
                    src[src_i + pcs.egaplaneofs[2] as usize],
                    src[src_i + pcs.egaplaneofs[3] as usize],
                ];
                vbuf[vbuf_i] = EGA(&chan, 7);
                vbuf_i += 1;
                vbuf[vbuf_i] = EGA(&chan, 6);
                vbuf_i += 1;
                vbuf[vbuf_i] = EGA(&chan, 5);
                vbuf_i += 1;
                vbuf[vbuf_i] = EGA(&chan, 4);
                vbuf_i += 1;
                vbuf[vbuf_i] = EGA(&chan, 3);
                vbuf_i += 1;
                vbuf[vbuf_i] = EGA(&chan, 2);
                vbuf_i += 1;
                vbuf[vbuf_i] = EGA(&chan, 1);
                vbuf_i += 1;
                vbuf[vbuf_i] = EGA(&chan, 0);

                src_i += 1;
                vbuf_i += screenpitch - 7;
            }
        }
    };
}

pub fn drawpic(
    x: i32,
    y: i32,
    picnum: i32,
    gs: &mut GlobalState,
    cps: &mut CpanelState,
    pcs: &mut PcrlibCState,
) {
    let vbuf = &mut gs.screenseg;
    let mut vbuf_i = y as usize * screenpitch + x as usize;
    let picwidth = cps.pictable[picnum as usize].width;
    let picheight = cps.pictable[picnum as usize].height;
    let src = &mut pcs.picfile_data;
    let mut src_i = pcs.picptr + cps.pictable[picnum as usize].shapeptr as usize;
    match pcs.grmode {
        CGAgr => {
            for _ in 0..picheight {
                for _ in 0..picwidth {
                    vbuf[vbuf_i] = (src[src_i] >> 6) & 3;
                    vbuf_i += 1;
                    vbuf[vbuf_i] = (src[src_i] >> 4) & 3;
                    vbuf_i += 1;
                    vbuf[vbuf_i] = (src[src_i] >> 2) & 3;
                    vbuf_i += 1;
                    vbuf[vbuf_i] = (src[src_i] >> 0) & 3;
                    vbuf_i += 1;

                    src_i += 1;
                }
                vbuf_i += screenpitch - (picwidth << 2) as usize;
            }
        }
        VGAgr => {
            // [BL] My best guess.
            for _ in 0..picheight {
                for _ in 0..picwidth {
                    vbuf[vbuf_i] = src[src_i];

                    src_i += 1;
                    vbuf_i += 1;
                }
                vbuf_i += screenpitch - picwidth as usize;
            }
        }
        EGAgr | _ => {
            for _ in 0..picheight {
                for _ in 0..picwidth {
                    let chan = [
                        src[src_i + pcs.egaplaneofs[0] as usize],
                        src[src_i + pcs.egaplaneofs[1] as usize],
                        src[src_i + pcs.egaplaneofs[2] as usize],
                        src[src_i + pcs.egaplaneofs[3] as usize],
                    ];
                    src_i += 1;

                    vbuf[vbuf_i] = EGA(&chan, 7);
                    vbuf_i += 1;
                    vbuf[vbuf_i] = EGA(&chan, 6);
                    vbuf_i += 1;
                    vbuf[vbuf_i] = EGA(&chan, 5);
                    vbuf_i += 1;
                    vbuf[vbuf_i] = EGA(&chan, 4);
                    vbuf_i += 1;
                    vbuf[vbuf_i] = EGA(&chan, 3);
                    vbuf_i += 1;
                    vbuf[vbuf_i] = EGA(&chan, 2);
                    vbuf_i += 1;
                    vbuf[vbuf_i] = EGA(&chan, 1);
                    vbuf_i += 1;
                    vbuf[vbuf_i] = EGA(&chan, 0);
                    vbuf_i += 1;
                }
                vbuf_i += screenpitch - (picwidth << 3) as usize;
            }
        }
    };
}
