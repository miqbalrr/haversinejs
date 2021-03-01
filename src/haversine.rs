use wasm_bindgen::prelude::*;

const R: f64 = 6371000.0;
const PHI: f64 = std::f64::consts::PI / 180.0;
const MILES: f64 = 1.60934;

pub trait Radian {
    fn radian(&self, src: f64) -> f64;
}

impl Radian for f64 {
    fn radian(&self, src: f64) -> Self {
        (self - src) * PHI
    }
}
pub trait Location {
    fn lat(&self) -> f64;
    fn lon(&self) -> f64;
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(lat: f64, lon: f64) -> Point {
        Self { lat, lon }
    }
}

impl Location for Point {
    fn lat(&self) -> f64 {
        self.lat
    }
    fn lon(&self) -> f64 {
        self.lon
    }
}

#[wasm_bindgen]
pub struct Haversine {
    pub src: Point,
    pub dst: Point,
    pub is_miles: bool,
}

impl Haversine {
    fn d_lat_per2(&self) -> f64 {
        self.dst.lat().radian(self.src.lat()) / 2.0
    }

    fn d_lon_per2(&self) -> f64 {
        self.dst.lon().radian(self.src.lon()) / 2.0
    }

    pub fn distance(&self) -> f64 {
        let a: f64 = self.d_lat_per2().sin() * self.d_lat_per2().sin()
            + self.dst.lat().radian(0.0).cos()
                * self.src.lat().radian(0.0).cos()
                * self.d_lon_per2().sin()
                * self.d_lon_per2().sin();

        let res: f64 = R * (2.0 * (a.sqrt()).atan2((1.0 - a).sqrt()));
        match self.is_miles {
            false => res,
            true => res / MILES,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn static_range() {
        let source = Point {
            lat: -7.0,
            lon: 7.0,
        };

        let destination = Point {
            lat: -8.0,
            lon: 8.0,
        };

        let haversine = Haversine {
            src: source,
            dst: destination,
            is_miles: false,
        };

        let result = haversine.distance();
        assert_eq!(result, 156581.10278754708);
    }
}
