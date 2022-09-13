use std::{convert::TryInto, io::Read, mem};

use serdine::derive::Deserialize;

use crate::spksndtype::spksndtype;

#[derive(Deserialize)]
pub struct SPKRtable {
    pub id: [i8; 4],
    pub filelength: u16,
    pub filler: [u16; 5],
    pub sounds: [spksndtype; 63],
    #[deserialize = "deserialize_freqdata"]
    pub freqdata: Vec<u16>,
}

fn deserialize_freqdata<R: Read>(mut r: R) -> Result<Vec<u16>, std::io::Error> {
    let mut buffer = Vec::new();
    r.read_to_end(&mut buffer).unwrap();
    let result = buffer
        .chunks_exact(2)
        .map(|word_bytes| u16::from_le_bytes(word_bytes.try_into().unwrap()))
        .collect();
    Ok(result)
}

impl Default for SPKRtable {
    fn default() -> Self {
        Self {
            id: Default::default(),
            filelength: Default::default(),
            filler: Default::default(),
            sounds: [spksndtype::default(); 63],
            freqdata: Default::default(),
        }
    }
}

impl SPKRtable {
    pub const fn start_of_freqdata() -> usize {
        mem::size_of::<[i8; 4]>()
            + mem::size_of::<u16>()
            + mem::size_of::<[u16; 5]>()
            + mem::size_of::<[spksndtype; 63]>()
    }
}
