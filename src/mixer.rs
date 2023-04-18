use crate::{vco::VCO, generator::Generator};

/// This is an object that spits out a left and right channel
/// it takes in every kind of audio producer and mixes to desired output
/// will eventually applys filters and stuff appropiately
pub struct Renderer {
    volume: f32,
    pub vcos: Vec<Box<dyn Generator<Item = f32>>>
}

// TODO implement panning
impl Renderer {
    pub fn new(volume: f32) -> Self {
        return Renderer { volume, vcos: Vec::new() }
    }

    pub fn out(&mut self) -> f32 {
        let mut sample = 0.;
        for vco in self.vcos.iter_mut() {
            sample += vco.next().unwrap();
        }
        return sample as f32 * self.volume;
    }
}