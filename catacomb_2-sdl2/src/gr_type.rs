use num::FromPrimitive;
use num_derive::FromPrimitive;
use serdine::derive::{Deserialize, Serialize};

/// In the original, this is an enum, stored as u16.
#[repr(u16)]
#[derive(Clone, Copy, Default, Deserialize, FromPrimitive, PartialEq, Serialize)]
pub enum grtype {
    VGAgr = 3,
    EGAgr = 2,
    CGAgr = 1,
    #[default]
    text = 0,
}

impl From<i32> for grtype {
    fn from(value: i32) -> Self {
        FromPrimitive::from_i32(value).unwrap()
    }
}

impl From<u16> for grtype {
    fn from(value: u16) -> Self {
        FromPrimitive::from_u16(value).unwrap()
    }
}
