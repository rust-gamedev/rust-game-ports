use serdine::derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct farptr {
    pub ofs: u16,
    pub seg: u16,
}

impl farptr {
    /// Rust port: Converted to isize for convenience.
    ///
    pub fn flatptr(&self) -> isize {
        (((self.seg as isize) << 4) + self.ofs as isize) as isize
    }
}

#[derive(Deserialize)]
pub struct picfiletype {
    pub charptr: farptr,
    pub tileptr: farptr, // Rust port: this is actually unused
    pub picptr: farptr,
    pub spriteptr: farptr,
    pub pictableptr: farptr,
    pub spritetableptr: farptr,
    pub plane: [farptr; 4],
    pub numchars: i16,
    pub numtiles: i16,
    pub numpics: i16,
    pub numsprites: i16,
}
