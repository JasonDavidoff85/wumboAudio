use std::f32::consts::PI;
use crate::generator::Generator;

type WaveShape = fn(f32, f32, f32) -> f32;

#[derive(Copy, Clone)]
pub struct VCO<T, U> {
    shape: WaveShape,
    time: f32,
    pub freq: T,
    pub volume: U
}

impl<T, U> VCO<T, U> {
    pub fn new(freq: T, volume: U, shape: WaveShape) -> Self {
        VCO {
            time: 0.,
            freq,
            volume,
            shape,
        }
    }
}

impl<T, U> Generator for VCO<T, U> where T: Generator<Item=f32>, U: Generator<Item=f32>{
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1. / 44_100.;
        let output = (self.shape)(self.freq.next().unwrap(), self.time, self.volume.next().unwrap());
        Some(output)
    }
}