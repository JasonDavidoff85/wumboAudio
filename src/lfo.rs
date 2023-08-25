use crate::generator::Generator;

type WaveShape = fn(f64, f64, f64) -> f64;

pub struct LFO {
    pub q: f64,
    pub rate: f64,
    pub width: f64,
    pub shape: WaveShape,
    time: f64
}

impl LFO {
    pub fn new(q: f64, rate: f64, width: f64, shape: WaveShape) -> Self {
        LFO {q, rate, width, shape, time: 0.}
    }
}

impl Generator for LFO {
    type Item = f64; // this means it can be sent to audio directly just be 32bit
    fn next(&mut self) -> Option<Self::Item> {
        // self.time += 1. / 44_100.;
        if self.time >= f64::MAX {
            self.time = 0f64;
        }
        else {
            self.time += 0.00002267573f64;
        }
        let output = (self.shape)(self.rate, self.time, self.width) + self.q;
        Some(output)
    }
}