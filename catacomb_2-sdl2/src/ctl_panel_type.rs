use serdine::derive::{Deserialize, Serialize};

use crate::{gr_type::grtype, sound_type::soundtype};

#[derive(Copy, Clone, Default, Deserialize, Serialize)]
pub struct ctlpaneltype {
    pub grmode: grtype,
    pub soundmode: soundtype,
    pub playermode: [u16; 3],
    pub JoyXlow: [i16; 3],
    pub JoyYlow: [i16; 3],
    pub JoyXhigh: [i16; 3],
    pub JoyYhigh: [i16; 3],
    pub MouseSensitivity: i16,
    pub key: [u8; 8],
    pub keyB1: u8,
    pub keyB2: u8,
}
