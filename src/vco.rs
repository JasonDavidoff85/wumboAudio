use std::f32::consts::PI;

use crate::generator::Generator;

type WaveShape = fn(f64, f64, f64) -> f64;

#[derive(Copy, Clone)]
pub struct VCO<T, U> {
    shape: WaveShape,
    time: f64,
    pub freq: T,
    pub volume: U
}

impl<T, U> VCO<T, U> {
    pub fn new(freq: T, volume: U, shape: WaveShape) -> Self {
        VCO {
            time: 0f64,
            freq,
            volume,
            shape,
        }
    }
}

impl<T, U> Generator for VCO<T, U> where T: Generator<Item=f64>, U: Generator<Item=f64>{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        // self.time = (self.time + (1. / 44_100.)) % f32::MAX;
        if self.time >= f64::MAX {
            self.time = 0f64;
        }
        else {
            self.time += 0.00002267573f64;
        }
        let output = (self.shape)(self.freq.next().unwrap(), self.time, self.volume.next().unwrap());
        Some(output as f32)
    }
}