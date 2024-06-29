use std::f64::consts::PI;
use num_traits::sign::signum;

/// add equations for waves here
pub fn sine(freq: f64, time: f64, volume: f64 ) -> f64 {
    return (freq * time * PI * 2.).sin() * volume;
}

pub fn square(freq: f64, time: f64, volume: f64 ) -> f64 {
    return signum((freq * time * PI * 2.).sin()) * volume;
}

pub fn saw(freq: f64, time: f64, volume: f64) -> f64 {
    return ((freq/2.) * time) % volume
}
