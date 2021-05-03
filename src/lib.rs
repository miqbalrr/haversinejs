mod haversine;

use haversine::{Haversine, HaversineParameter, Point};
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

#[wasm_bindgen]
pub fn haversines(points: JsValue) -> js_sys::Float64Array {
    let points = points.into_serde::<Vec<HaversineParameter>>();
    if let Ok(points) = points {
        let mut results: Vec<f64> = vec![];
        points.iter().for_each(|param| {
            let h = Haversine {
                src: param.src,
                dst: param.dst,
                is_miles: param.is_miles,
            };
            results.push(h.distance())
        });
        js_sys::Float64Array::from(results.as_ref())
    } else {
        js_sys::Float64Array::from(vec![].as_ref())
    }
}
