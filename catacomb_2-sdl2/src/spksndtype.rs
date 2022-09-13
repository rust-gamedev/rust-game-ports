use serdine::derive::Deserialize;

#[derive(Copy, Clone, Default, Deserialize, Debug)]
pub struct spksndtype {
    pub start: u16,
    pub priority: u8,
    pub samplerate: u8,
    pub name: [i8; 12],
}
