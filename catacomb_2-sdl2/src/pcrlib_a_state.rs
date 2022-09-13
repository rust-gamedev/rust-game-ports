use crate::{
    pcrlib_a::SavedSoundStruct, sound_type::soundtype, sound_type::soundtype::*,
    spkr_table::SPKRtable,
};
use std::sync::{Arc, Mutex};

/// (Rust port)
/// Globals previously belonging to pcrlib_a.rs.
///
/// This is the type gated by the mutex; functions accept it as parameter, in cases where the mutex
/// has already been acquired (this makes a nice distinction between functions that need to acquire
/// the mutex, and those who assume that this has been done).
#[rustfmt::skip]
pub struct PcrlibAStateExclusive {
    // //////////////////////////////////////////////////////////
    // Rust port: shared
    // //////////////////////////////////////////////////////////

    pub SoundData: SPKRtable,
    pub soundmode: soundtype,

    // //////////////////////////////////////////////////////////
    // Rust port: private to pcrlib_a.rs
    // //////////////////////////////////////////////////////////

    pub SndPriority: u8,

    // Rust port: The audio mutex has been moved to be in the `pcrlib_a` module scope, in order to
    // avoid borrowing contention on the PcrlibAState instance.

    // Rust port: In the SDL port, the desired audio spec and device are stored, but only the frequency
    // is used, so we only store that. Additionally, the port assumed that the obtained spec is the
    // same as the desired, so this is assumed here as well.

    pub AudioSpecFreq: i32,
    pub pcVolume: i16,
    pub pcPhaseTick: u32,
    pub pcPhaseLength: u32,
    pub pcActive: bool,
    pub pcSamplesPerTick: u32,
    pub pcLastSample: u16,
    pub pcLengthLeft: u32,
    // Rust port: Pointer to SoundData.freqdata
    pub pcSound: Option<usize>,
    pub SavedSound: SavedSoundStruct,

    // //////////////////////////////////////////////////////////
    // Rust port: private to cpanel.rs
    // //////////////////////////////////////////////////////////

    // pub xormask: i32, // Rust port: Set but never read
}

/// Threadsafe wrapper around the actual PrclibAState data type.
#[rustfmt::skip]
#[derive(Clone)]
pub struct PcrlibAState {
    inner: Arc<Mutex<PcrlibAStateExclusive>>,
    // Rust port: the following are behind a smart pointer, and on cloning, they will be duplicated.
    // This is acceptable, because:
    // - _dontplay is set before cloning, and after, it's used only within the sound thread;
    // - the random vars are used only in the main thread (they could even be split).

    // //////////////////////////////////////////////////////////
    // Rust port: private to pcrlib_a.rs
    // //////////////////////////////////////////////////////////

    pub _dontplay: bool,

    pub rndindex: u16,
    pub indexi: u16,
    pub indexj: u16,
    pub LastRnd: u16,
    pub RndArray: [u16; 17],
}

impl PcrlibAState {
    pub fn new() -> Self {
        let inner = PcrlibAStateExclusive {
            SndPriority: 0,
            AudioSpecFreq: 0,
            pcVolume: 5000,
            pcPhaseTick: 0,
            pcPhaseLength: 0,
            pcActive: false,
            pcSamplesPerTick: 0,
            pcLastSample: 0,
            pcLengthLeft: 0,
            pcSound: None,
            SavedSound: SavedSoundStruct {
                SndPriority: 0,
                pcSamplesPerTick: 0,
                pcLengthLeft: 0,
                pcSound: None,
            },
            SoundData: SPKRtable::default(),
            soundmode: spkr,
        };

        Self {
            inner: Arc::new(Mutex::new(inner)),
            _dontplay: false,
            rndindex: 0,
            indexi: 0,
            indexj: 0,
            LastRnd: 0,
            RndArray: [0; 17],
        }
    }
}

impl PcrlibAState {
    pub fn lock<R, F: FnMut(&mut PcrlibAStateExclusive) -> R>(&self, mut fx: F) -> R {
        let mut lock = (*self.inner).lock().unwrap();
        fx(&mut lock)
    }
}
