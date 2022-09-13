/// In the original, this is an enum, (automatically) narrowed down to u8 on assignment.
#[repr(u8)]
pub enum thinktype {
    gunthinks = 10,
    gunthinke = 9,
    explode = 8,
    fade = 7,
    idle = 6,
    straight = 5,
    ramdiag = 4,
    ramstraight = 3,
    dragoncmd = 2,
    gargcmd = 1,
    playercmd = 0,
}
