use std::cmp;

use crate::prelude::*;

//# Used when calling functions such as sorted and min.
//# todo explain more
//# p.vpos - pos results in a Vector2 which we can get the length of, giving us
//# the distance between pos and p.vpos
//
// In the port, this takes the vpos's as input, in order to match the rust iterator math APIs.
//
pub fn dist_key(vpos1: &Vector2<f32>, vpos2: &Vector2<f32>, pos: Vector2<f32>) -> cmp::Ordering {
    let p1_norm = (vpos1 - pos).norm();
    let p2_norm = (vpos2 - pos).norm();

    p1_norm.partial_cmp(&p2_norm).unwrap()
}

//# Turn a vector into a unit vector - i.e. a vector with length 1
//# We also return the original length, before normalisation.
//# We check for zero length, as trying to normalise a zero-length vector results in an error
pub fn safe_normalise(vec: &Vector2<f32>) -> (Vector2<f32>, f32) {
    let length = vec.norm();

    if length == 0.0 {
        (Vector2::new(0.0, 0.0), 0.0)
    } else {
        (vec.normalize(), length)
    }
}
