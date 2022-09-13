use std::iter;

// Rust port: Not intended to be fast; the substance is pretty much the same, the difference being
// that indices are used, rather than pointers.
pub fn RLEExpand(source: &[u8], source_len: usize, dest: &mut Vec<u8>) {
    let mut source_i = 0;

    // Rust port: there was a bug here in the SDL port (potentially also in the original project,
    // but didn't check); the dest pointer was checked, which caused buffer overruns.
    // There was also a bug in the original project; the destination buffer is 4096 bytes long, while
    // the largest level expands to 4338 bytes.
    while source_i < source_len {
        let val: u8 = source[source_i];
        source_i += 1;

        if val & 0x80 != 0 {
            let len = ((val & 0x7f) + 1) as usize;
            dest.extend(&source[source_i..source_i + len]);
            source_i += len;
        } else {
            let len = (val + 3) as usize;
            dest.extend(iter::repeat(source[source_i]).take(len));
            source_i += 1;
        }
    }
}
