use crate::dir_type::dirtype;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ControlStruct {
    pub dir: dirtype,
    pub button1: bool,
    pub button2: bool,
}
