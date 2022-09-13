use std::io::Write;
use std::{convert::TryInto, io::Read};

use serdine::derive::{Deserialize, Serialize};
use serdine::Serialize as SerialzeTraits;

// Rust port: WATCH OUT! On disk, this has a different structure (see #initials and related methods).
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct scores {
    pub score: i32,
    pub level: i16,
    #[deserialize = "deserialize_initials"]
    #[serialize = "serialize_initials"]
    pub initials: [u8; 3],
}

fn deserialize_initials<R: Read>(mut r: R) -> Result<[u8; 3], std::io::Error> {
    let mut buffer = [0; 4];
    r.read_exact(&mut buffer).unwrap();
    let result = buffer[0..3].try_into().unwrap();
    Ok(result)
}

fn serialize_initials<W: Write>(instance: &[u8; 3], mut w: W) -> Result<(), std::io::Error> {
    instance.serialize(&mut w)?;
    w.write_all(&[0])?;
    Ok(())
}

impl scores {
    pub const fn ondisk_struct_size() -> usize {
        10
    }
}
