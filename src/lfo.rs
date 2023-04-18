use std::f64::consts::PI;

use crate::generator::Generator;

pub struct LFO {
    pub rate: f64,
    pub width: f64,
    time: f64
}

impl LFO {
    pub fn new(rate: f64, width: f64) -> Self {
        LFO {rate, width, time: 0.}
    }
}

impl Generator for LFO {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1. / 44_100.;
        let output = ((self.rate as f64 * self.time * PI * 2.).sin() * self.width as f64) as Self::Item;
        Some(output)
    }
}