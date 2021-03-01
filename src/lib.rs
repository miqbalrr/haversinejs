mod haversine;

use haversine::{Haversine, Point};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn haversine(src: Point, dst: Point, is_miles: bool) -> f64 {
    let h = Haversine { src, dst, is_miles };
    h.distance()
}
