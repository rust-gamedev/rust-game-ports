use std::cmp;

use crate::prelude::*;

//# Used when calling functions such as sorted and min.
//# todo explain more
//# p.vpos - pos results in a Vector2 which we can get the length of, giving us
//# the distance between pos and p.vpos
//
// In the port, this takes the vpos's as input, in order to match the rust iterator math APIs.
//
pub fn dist_key(vpos1: &Vector2<i16>, vpos2: &Vector2<i16>, pos: Vector2<i16>) -> cmp::Ordering {
    // Recreating the vector is ugly; it's due to the port using u16 as standard unit, for simplicity.
    // Currently, it's overall simpler to relegate the conversions here, rather than mixing f32 with
    // u16. The alternative is to use f32 everywhere.
    //
    let p1_norm = Vector2::new((vpos1.x - pos.x) as f32, (vpos1.y - pos.y) as f32).norm();
    let p2_norm = Vector2::new((vpos2.x - pos.x) as f32, (vpos2.y - pos.y) as f32).norm();

    p1_norm.partial_cmp(&p2_norm).unwrap()
}

//# Turn a vector into a unit vector - i.e. a vector with length 1
//# We also return the original length, before normalisation.
//# We check for zero length, as trying to normalise a zero-length vector results in an error
pub fn safe_normalise(vec: &Vector2<i16>) -> (Vector2<f32>, f32) {
    let vec = Vector2::new(vec.x as f32, vec.y as f32);
    let length = vec.norm();

    if length == 0.0 {
        (Vector2::new(0.0, 0.0), 0.0)
    } else {
        (vec.normalize(), length)
    }
}
