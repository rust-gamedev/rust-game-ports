use num::FromPrimitive;
use num_derive::FromPrimitive;

#[repr(u32)]
#[derive(Clone, Copy, FromPrimitive)]
pub enum dirtype {
    nodir = 8,
    northwest = 7,
    southwest = 6,
    southeast = 5,
    northeast = 4,
    west = 3,
    south = 2,
    east = 1,
    north = 0,
}

// For readability. Possibly, only a reference one will be needed once/if the data types are fully
// cleaned up.

impl From<i32> for dirtype {
    fn from(value: i32) -> Self {
        FromPrimitive::from_i32(value).unwrap()
    }
}

impl From<u16> for dirtype {
    fn from(value: u16) -> Self {
        FromPrimitive::from_u16(value).unwrap()
    }
}

impl From<u8> for dirtype {
    fn from(value: u8) -> Self {
        FromPrimitive::from_u8(value).unwrap()
    }
}
