use std::{cmp, f32::consts::PI};

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

//# Custom sine/cosine functions for angles of 0 to 7, where 0 is up,
//# 1 is up+right, 2 is right, etc.
pub fn sin(x: u8) -> f32 {
    (x as f32 * PI / 4.).sin()
}

pub fn cos(x: u8) -> f32 {
    sin(x + 2)
}

//# Convert a vector to an angle in the range 0 to 7
pub fn vec_to_angle(vec: Vector2<f32>) -> u8 {
    //# todo explain a bit
    //# https://gamedev.stackexchange.com/questions/14602/what-are-atan-and-atan2-used-for-in-games
    (4. * vec.x.atan2(-vec.y) / PI + 8.5) as u8 % 8
}

//# Convert an angle  in the range 0 to 7 to a direction vector. We use -cos rather than cos as increasing angles move
//# in a clockwise rather than the usual anti-clockwise direction.
pub fn angle_to_vec(angle: u8) -> Vector2<f32> {
    Vector2::new(sin(angle), -cos(angle))
}
