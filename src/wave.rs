use std::f32::consts::PI;
use num_traits::sign::signum;

/// add equations for waves here
pub fn sine(freq: f32, time: f32, volume: f32 ) -> f32 {
    return (freq * time * PI * 2.).sin() * volume;
}

pub fn square(freq: f32, time: f32, volume: f32 ) -> f32 {
    return signum((freq * time * PI * 2.).sin()) * volume;
}

pub fn saw(freq: f32, time: f32, volume: f32) -> f32 {
    return ((freq/2.) * time) % volume
}
