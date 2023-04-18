use std::f64::consts::PI;
use crate::generator::Generator;

type WaveShape = fn(f64, f64, f64) -> f32;

#[derive(Copy, Clone)]
pub struct VCO<T> {
    shape: WaveShape,
    time: f64,
    pub freq: T,
    volume: f64
}

impl<T> VCO<T> {
    pub fn new(freq: T, volume: f64, shape: WaveShape) -> Self {
        VCO {
            time: 0.,
            freq,
            volume,
            shape,
        }
    }
}

impl<T> Generator for VCO<T> where T: Generator<Item=f64> {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1. / 44_100.;
        let output = (self.shape)(self.freq.next().unwrap(), self.time, self.volume);
        Some(output)
    }
}