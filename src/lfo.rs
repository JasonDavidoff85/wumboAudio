use std::f32::consts::PI;

use crate::generator::Generator;

pub struct LFO {
    pub rate: f32,
    pub width: f32,
    time: f32
}

impl LFO {
    pub fn new(rate: f32, width: f32) -> Self {
        LFO {rate, width, time: 0.}
    }
}

impl Generator for LFO {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1. / 44_100.;
        let output = ((self.rate * self.time * PI * 2.).sin() * self.width) as Self::Item;
        Some(output)
    }
}