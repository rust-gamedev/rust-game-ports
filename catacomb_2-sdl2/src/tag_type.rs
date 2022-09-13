/// In the original, this is an enum, (automatically) narrowed down to u8 on assignment.
#[repr(u8)]
pub enum tagtype {
    nukeshot = 4,
    mshot = 3,
    pshot = 2,
    monster = 1,
    benign = 0,
}
