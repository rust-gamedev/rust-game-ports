use num::FromPrimitive;
use num_derive::FromPrimitive;

#[derive(Clone, Copy, FromPrimitive, PartialEq)]
#[repr(u16)]
pub enum inputtype {
    demo = 4,
    joystick2 = 3,
    joystick1 = 2,
    mouse = 1,
    keyboard = 0,
}

// For readability. Possibly, only a reference one will be needed once/if the data types are fully
// cleaned up.

impl From<i32> for inputtype {
    fn from(value: i32) -> Self {
        FromPrimitive::from_i32(value).unwrap()
    }
}

impl From<u16> for inputtype {
    fn from(value: u16) -> Self {
        FromPrimitive::from_u16(value).unwrap()
    }
}
