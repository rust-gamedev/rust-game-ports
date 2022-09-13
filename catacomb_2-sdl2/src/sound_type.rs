use num::FromPrimitive;
use num_derive::FromPrimitive;
use serdine::derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, FromPrimitive, PartialEq, Serialize)]
#[repr(u16)]
pub enum soundtype {
    sdlib = 2,
    spkr = 1,
    #[default]
    off = 0,
}

// For readability. Possibly, only a reference one will be needed once/if the data types are fully
// cleaned up.

impl From<i32> for soundtype {
    fn from(value: i32) -> Self {
        FromPrimitive::from_i32(value).unwrap()
    }
}

impl From<u16> for soundtype {
    fn from(value: u16) -> Self {
        FromPrimitive::from_u16(value).unwrap()
    }
}
