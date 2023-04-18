use std::f64::consts::PI;
use num_traits::sign::signum;

/// add equations for waves here

pub fn sine(freq: f64, time: f64, volume: f64 ) -> f32 {
    return ((freq * time * PI * 2.).sin() * volume) as f32;
}

pub fn square(freq: f64, time: f64, volume: f64 ) -> f32 {
    return (signum((freq * time * PI * 2.).sin()) * volume) as f32;
}
