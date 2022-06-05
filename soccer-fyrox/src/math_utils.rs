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
