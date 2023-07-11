use num_traits::Pow;

use crate::generator::Generator;
use std::f64::consts::PI;

/// Using the Direct form 1 of the biquad linear filter formula.  
/// source: https://www.w3.org/TR/audio-eq-cookbook/#formulae
pub struct Filter {
    sample_rate: f64,
    center_freq: f64,
    q: f64,
    gain: f64,
    a: f64,
    w0: f64,
    alpha: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    a0: f64,
    a1: f64,
    a2: f64,
    xmem1: f64,
    xmem2: f64,
    ymem1: f64,
    ymem2: f64,
}

fn peaking_eq(a: f64, q: f64, w0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let c = w0.cos();
    let s = w0.sin();
    let alpha = s * ((2f64.log10() / 2.) * q * (w0 / s)).sinh();
    let coe = 1. + (alpha/a);
    let a1 = (-2. * c) / coe;
    let a2 = (1. - (alpha/a)) / coe;
    let b0 = (1. + (alpha*a)) / coe;
    let b1 = (-2. * c) / coe;
    let b2 = (1. - (alpha*a)) / coe;
    (alpha, a1, a2, b0, b1, b2)
}

fn low_pass(a: f64, q: f64, w0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let c = w0.cos();
    let s = w0.sin();
    let alpha = (s / 2.) * (((a + (1. / a)) * ((1. / q) - 1.)) + 2.).sqrt();
    let coe = 1. + a;
    let a1 = (-2. * c) / coe;
    let a2 = (1. - a) / coe;
    let b0 = ((1. - c) / 2.) / coe;
    let b1 = (1. - c) / coe;
    let b2 = ((1. - c) / 2.) / coe;
    (alpha, a1, a2, b0, b1, b2)
}

impl Filter {
    pub fn new(center_freq: f64, q: f64, gain: f64) -> Self {
        let sample_rate = 44_100.;
        let a = 10f64.pow(gain / 40.);
        let w0 = 2. * PI * (center_freq / sample_rate);
        let a0 = 1.;
        let (alpha, a1, a2, b0, b1, b2) = low_pass(a, q, w0);
        
        return Filter {
            sample_rate,
            center_freq,
            q,
            gain,
            a,
            w0,
            alpha,
            b0,
            b1,
            b2,
            a0,
            a1,
            a2,
            xmem1: 0.,
            xmem2: 0.,
            ymem1: 0.,
            ymem2: 0.,
        }
    }

    pub fn next(&mut self, val: f64) -> f64 {
        let y = (self.b0 * val) + (self.b1 * self.xmem1) + (self.b2 * self.xmem2) - (self.a1 * self.ymem1) - (self.a2 * self.ymem2);
        self.xmem2 = self.xmem1;
        self.xmem1 = val;
        self.ymem2 = self.ymem1;
        self.ymem1 = y;
        y
    }
}

